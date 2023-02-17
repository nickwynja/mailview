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
    // On click, open link in browser
    Anchors[i].addEventListener("click",
      function (event) {
        event.preventDefault();
        if(event.metaKey) {
          external.invoke(`bg ${this.href}`);
        } else {
          external.invoke(this.href);
        }
      },
      false);

    // set title attr for hovering on links
    Anchors[i].title = Anchors[i].href;
  }
})
