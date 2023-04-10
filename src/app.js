document.onkeydown = function(event) {
  if (document.activeElement.tagName === "INPUT") {
    return;
  }
  switch (event.key) {
    case 'c':
      if(event.metaKey) {
        return document.execCommand('copy');
      }
      break;
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
    case '/':
      event.preventDefault();
      showSearch();
      break;
  }
};

document.onkeyup = function(event) {
  if (document.activeElement.tagName === "INPUT") {
    switch (event.key) {
      case 'Enter':
        var i = document.getElementById('search-input')
        found = window.find(i.value, false, false, true, false);
        break;
      case '/':
        event.preventDefault();
        showSearch();
        break;
    }
  }
}

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
  addSearch();
})


function addSearch() {
  const div = document.createElement("div");
  div.id = "search" ;
  const input = document.createElement("input");
  input.id = "search-input";
  input.setAttribute('autocomplete', 'off')
  input.setAttribute('autocorrect', 'off')
  input.setAttribute('autocapitalize', 'off')
  input.setAttribute('spellcheck', false)
  div.appendChild(input);

  document.body.appendChild(div);
}

function showSearch() {
  const s = document.getElementById('search')
  const i = document.getElementById('search-input')
  console.log(s.classList);
  if (s.classList.contains('open')) {
    i.value = ""
  } else {
    s.classList.add('open');
  }
  i.focus();
}

function hideSearch() {
  const s = document.getElementById('search')
  s.classList.remove('open');
}
