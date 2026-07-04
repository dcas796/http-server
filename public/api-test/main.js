function make_request() {
    const message_input = document.querySelector("#message-input");
    const message = message_input.value;
    fetch("/api/echo?m=" + encodeURIComponent(message))
        .then(response => response.json())
        .then(json => {
            const response_container = document.querySelector("#response-container");
            const response_text = document.querySelector("#response-text");
            if (json.status === "success" && json.message !== undefined) {
                response_text.innerHTML = decodeURIComponent(json.message);
            } else {
                response_text.innerHTML = JSON.stringify(json)
            }
            response_container.style.display = "block";
        })
}