document.getElementById('fetchFiles').onclick = async function () {
    const repoUrl = document.getElementById('repoUrl').value;
    const fileListElement = document.getElementById('fileList');
    fileListElement.innerHTML = '';

    const match = repoUrl.match(/github\.com\/([^\/]+)\/([^\/]+)/);
    console.log(match);
    if (!match) {
        alert('Invalid GitHub URL');
        return;
    }

    const [_, owner, repo] = match;
    const apiUrl = `https://api.github.com/repos/${owner}/${repo}/contents`;

    try {
        const response = await fetch(apiUrl);
        if (!response.ok) {
            throw new Error(`Error fetching repository contents: ${response.statusText}`);
        }

        const files = await response.json();
        files.forEach(file => {
            const listItem = document.createElement('li');
            const link = document.createElement('a');
            link.href = `https://github.com/${owner}/${repo}/blob/main/${file.path}`; // need to change the link to also get the branch name (if main doesnt exist it will send to a 404 not found)
            link.target = '_blank'; // Open link in a new tab
            link.textContent = file.path;
            listItem.appendChild(link);
            fileListElement.appendChild(listItem);
        });
    } catch (error) {
        alert('Failed to fetch repository contents: ' + error.message);
    }
};

document.getElementById('deployButton').onclick = function () {
    window.location.href = 'https://boxer.dev';
};