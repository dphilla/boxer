let wasmInstance = null;
let memoryBuffer = null;

let stdoutBuffer = "";

let websocket = null;
let nextRequestId = 1;
const pendingRequests = new Map();

//  Listen for messages from main thread
onmessage = async (evt) => {
  const { cmd, payload } = evt.data;

  switch (cmd) {
    case 'initWasm':
      await initWasm(payload.wasmUrl);
      postMessage({ cmd: 'wasmReady' });
      break;

    case 'callMain':
      if (!wasmInstance) return;
      const ret = wasmInstance.exports._start ? wasmInstance.exports._start() : 0;
      postMessage({ cmd: 'callMainDone', returnValue: ret });
      break;

    default:
      console.log("[Worker] Unknown cmd:", cmd);
      break;
  }
};

function initWebSocket() {
  return new Promise((resolve, reject) => {
    websocket = new WebSocket('ws://127.0.0.1:9000');
    websocket.binaryType = 'arraybuffer';

    websocket.onopen = () => {
      console.log('[Worker] WebSocket connected');
      resolve();
    };

    websocket.onerror = (err) => {
      console.error('[Worker] WebSocket error:', err);
      reject(new Error('WebSocket connection error'));
    };

    websocket.onmessage = (evt) => {
      // We assume each message is: [4 bytes requestId][... arbitrary data ...]
      const fullData = new Uint8Array(evt.data);
      if (fullData.length < 4) {
        console.warn('Received message too short');
        return;
      }
      const view = new DataView(fullData.buffer);
      const reqId = view.getInt32(0, true); // little-endian
      const payload = fullData.subarray(4);

      const reqInfo = pendingRequests.get(reqId);
      if (!reqInfo) {
        console.warn('Received response for unknown request id', reqId);
        return;
      }
      // Store the payload in reqInfo, then do an Atomics.notify
      reqInfo.responseData = payload;
      Atomics.store(reqInfo.sab, 0, 1);
      Atomics.notify(reqInfo.sab, 0, 1);
    };

    websocket.onclose = () => {
      console.log('[Worker] WebSocket closed');
    };
  });
}

async function initwasm(wasmurl) {
  // create websocket (only if /net is needed)
  await initwebsocket();

  const response = await fetch(wasmurl);
  const bytes = await response.arraybuffer();

  const memory = new webassembly.memory({ initial: 256, maximum: 256 });
  memorybuffer = memory.buffer;

  function box_host_write_stdout_line(ptr, len) {
    const bytes = new Uint8Array(memoryBuffer, ptr, len);
    const line = new TextDecoder('utf-8').decode(bytes);
    postMessage({ cmd: 'hostWriteStdout', text: line });
  }

  function blockyr(reqptr, reqlen, outptr) {
    const requ8 = new uint8array(memorybuffer, reqptr, reqlen);
    const reqid = nextrequestid++;

    // build [reqid + request data]
    const header = new uint8array(4);
    new dataview(header.buffer).setint32(0, reqid, true);
    const tosend = new uint8array(4 + reqlen);
    tosend.set(header, 0);
    tosend.set(requ8, 4);

    const sab = new int32array(new sharedarraybuffer(4));
    sab[0] = 0; // 0 => not ready
    pendingrequests.set(reqid, { sab, responsedata: null });

    websocket.send(tosend);

    // "block" via atomics until the server replies
    while (atomics.load(sab, 0) === 0) {
      atomics.wait(sab, 0, 0);
    }

    const { responsedata } = pendingrequests.get(reqid);
    pendingrequests.delete(reqid);

    if (!responsedata) {
      new dataview(memorybuffer).setint32(outptr, 0, true);
      return 0;
    }

    // we have responsedata; allocate space in wasm memory
    const resplen = responsedata.length;
    const malloc = wasminstance.exports.malloc; // assumed exported by your minimal libc
    const respptr = malloc(resplen);

    const respmem = new uint8array(memorybuffer, respptr, resplen);
    respmem.set(responsedata);

    new dataview(memorybuffer).setint32(outptr, resplen, true);
    return respptr;
  }

  const env = {
    memory,
    box_host_write_stdout_line,
    r: blockyr
  };

  const { instance } = await webassembly.instantiate(bytes, { env });
  wasminstance = instance;

  console.log('[worker] wasm loaded & instance created.');
}

