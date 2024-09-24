document.body.addEventListener("htmx:responseError", function (event) {
  if (event.detail.xhr.status === 401) {
    window.location.reload();
  }
});

document.addEventListener("htmx:load", function () {
  feather.replace();
});
