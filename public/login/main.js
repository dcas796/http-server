function login() {
    const usernameEl = document.querySelector("input#username-input")
    const passwordEl = document.querySelector("input#password-input")
    const username = encodeURIComponent(usernameEl.value)
    const password = encodeURIComponent(passwordEl.value)

    fetch(
        `/api/login`,
        {
            method: "POST",
            body: `username=${username}\npassword=${password}`
        }
    )
        .then(response => response.json())
        .then(json => {
            const response_container = document.querySelector("#response-container");
            const response_text = document.querySelector("#response-text");
            if (json.message !== undefined) {
                response_text.innerHTML = decodeURIComponent(json.message);
            } else {
                response_text.innerHTML = JSON.stringify(json)
            }
            response_container.style.display = "block";
        })
}
