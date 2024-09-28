document.getElementById("fetchDataBtn").addEventListener("click", () => {
  fetch("http://127.0.0.1:7878/")
    .then((response) => {
      if (!response.ok) {
        throw new Error(`HTTP error! Status: ${response.status}`);
      }

      return response.json();
    })
    .then((data) => {
      console.log({ data });
      document.getElementById("output").innerHTML = `<pre>${JSON.stringify(
        data,
        null,
        2
      )}</pre>`;
    })
    .catch((error) => {
      document.getElementById("output").innerHTML = `Error: ${error.message}`;
    });
});

function postData() {
  const data = { key: "value" };

  fetch("http://127.0.0.1:7878", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(data),
  })
    .then((response) => {
      if (!response.ok) {
        throw new Error(`HTTP error! Status: ${response.status}`);
      }
      return response.json();
    })
    .then((data) => {
      document.getElementById("output").innerHTML = `<pre>${JSON.stringify(
        data,
        null,
        2
      )}</pre>`;
    })
    .catch((error) => {
      document.getElementById("output").innerHTML = `Error: ${error.message}`;
    });
}
