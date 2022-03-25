async function get_github_repository_stats(repository_owner, repository_name) {
    return fetch(`https://api.github.com/repos/${repository_owner}/${repository_name}`).then((response) => {
        if (response.ok === true) {
            return response.json();
        }
        throw new Error();
    }).then((response_json) => {
        return {
            stars: Number.parseInt(response_json.stargazers_count),
            forks: Number.parseInt(response_json.forks),
        }
    }).catch(() => {
        return {
            stars: 0,
            forks: 0,
        };
    })
}

async function update_fork_and_star_count_of_projects() {
    const canonical_names = [];
    const api_request_promises = [];
    const project_previews = document.querySelectorAll(".project-full figure");

    for (const project_container of project_previews) {
        const title_element = project_container.querySelector("strong");
        const canonical_name = title_element.dataset.canonicalName;
        canonical_names.push(canonical_name);
        const api_request_promise = get_github_repository_stats("kruhlmann", canonical_name).then((metadata) => {
            
        });
    }
}
