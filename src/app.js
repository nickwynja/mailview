document.onkeydown = function(event) {
  switch (event.key) {
    case 'q':
      external.invoke('exit')
      break;
    case 'G':
      window.scrollTo(0, 0)
      break;
    case 'g':
      window.scrollTo(0, document.body.scrollHeight)
      break;
    case 'k':
      window.scrollBy(0, -50);
      break;
    case 'j':
      window.scrollBy(0, 50);
      break;
    case 'Escape':
      external.invoke('exit')
      break;
  }
};

document.addEventListener("DOMContentLoaded", function() {
  var Anchors = document.getElementsByTagName("a");

  for (var i = 0; i < Anchors.length ; i++) {
    Anchors[i].addEventListener("click",
      function (event) {
        event.preventDefault();
        external.invoke(this.href);
      },
      false);
  }

// @TODO: https://github.com/webview/webview/issues/44#issuecomment-350342541
  //      https://github.com/Boscop/web-view/issues/170
//        Properly load local cid images

  var images = document.getElementsByTagName('img');

  for (var i = 0; i < images.length; i++) {
    if (images[i].src.startsWith('/')) {
      console.log(images[i]);
      // images[i].src = "file://" + images[i].src;
    }
  }
})