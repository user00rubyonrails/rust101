let createBtn = document.getElementById("create-btn");
createBtn.addEventListener("click", postAlert);

function postAlert() {
  let titleInput = document.getElementById("name");
  alert(titleInput.value);
  titleInput.value = null;
}