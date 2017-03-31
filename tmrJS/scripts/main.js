(function() {
  'use strict';

  var app = {
    main : document.querySelector('.main'),
    pages: {}
  }

  /***********************************************************************
  * ROUTE
  *
  ***********************************************************************/
  app.loadPageAt = function(route){
    if(app.pages[route]){
      app.displayPageContent(app.pages[route]);
    } else {
      fetch(route+'.html')
      .then(res => res.text())
      .then(content => {
        app.displayPageContent(content);
        app.pages[route] = content
      })
    }
  }

  app.displayPageContent = function(content){
    app.main.textContent = content
  }
  /***********************************************************************
  * INIT
  *
  ***********************************************************************/

  document.addEventListener('DOMContentLoaded', function(){
    var home = document.querySelector('.main').textContent
    app.pages[''] = home;
    //app.loadPageAt('')
  })

})();
