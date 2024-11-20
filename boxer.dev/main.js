document.addEventListener('DOMContentLoaded', () => {
    const modePreference = window.matchMedia("(prefers-color-scheme: dark");
    if (modePreference.matches) {
        document.getElementById('view-mode').checked = true;
        viewMode();
    }
});

document.getElementById('fetchFiles').onclick = async function () {
    const repoUrl = document.getElementById('repoUrl').value;
    gitHubFiles(repoUrl);
};

document.getElementById('repoUrl').addEventListener('keydown', async function (e) {
    if (e.key === 'Enter') {
        const repoUrl = document.getElementById('repoUrl').value;
        gitHubFiles(repoUrl);
    }
});

document.getElementById('view-mode').addEventListener("click", () => {
    viewMode();
});

// document.getElementById('deployButton').onclick = function () {
//     window.location.href = 'https://boxer.dev';
// };

const gitHubFiles = async (repoUrl) => {
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

const dockerfileLang = (language) => {
    switch (true) {
        case (language === 'python'):
            document.getElementById('dockerText').value = "# From Dockers official Python image: https://hub.docker.com/_/python\n\nFROM python:3\n\nWORKDIR /usr/src/app\n\nCOPY requirements.txt ./\nRUN pip install --no-cache-dir -r requirements.txt\n\nCOPY . .\n\nCMD [ 'python', './your-daemon-or-script.py' ]\n";
            break;
        case (language === 'ruby'):
            document.getElementById('dockerText').value = "# From Dockers official Ruby image: https://hub.docker.com/_/ruby\n\nFROM ruby:3.3\n\n# Throw errors if Gemfile has been modified since Gemfile.lock\nRUN bundle config --global frozen 1\n\nWORKDIR /usr/src/app\n\nCOPY Gemfile Gemfile.lock ./\nRUN bundle install\n\nCOPY . .\n\nCMD [ './your-daemon-or-script.rb' ]\n";
            break;
        default:
            document.getElementById('dockerText').value = "# From Dockers official GCC image: https://hub.docker.com/_/gcc\n\nFROM gcc:4.9\nCOPY . /usr/src/myapp\nWORKDIR /usr/src/myapp\nRUN gcc -o myapp main.c\nCMD ['./myapp']\n";

    }
};

const viewMode = () => {
    document.body.classList.toggle('dark-mode');
    if (document.getElementById('view-mode').checked) {
        document.getElementById('github-icon').src = "img/github-dark.png";
    } else {
        document.getElementById('github-icon').src = "img/github-light.png";
    }
};