if(document.getElementById("hamburger-menu") && document.getElementById("responsive-menu-items")) {
  const hamburgerMenu = document.getElementById("hamburger-menu");
  const menuItemsContainer = document.getElementById("responsive-menu-items");
  hamburgerMenu.onclick = function() {
    hamburgerMenu.classList.toggle("menu-open");
    menuItemsContainer.classList.toggle("menu-open");
  };
}