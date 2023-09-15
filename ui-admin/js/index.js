'use strict';

var ifNotAuthenticated = (to, from, next) => {
  if (!$storageService.get('auth-token')) {
    next();
    return
  }
  next('/')
};

var ifAuthenticated = (to, from, next) => {
  if ($storageService.get('auth-token')) {
    next();
  } else {
    next({ path: '/login', query: { from: to.path } });
  }

};


const router = new VueRouter({
  routes: [
    { path: '/', redirect: '/stock-builder'},
    // { path: '/',
    //   // beforeEnter: ifAuthenticated,
    //   component: Vue.component('admin'),
    //   children: [
    //     {
    //       path: 'pcc-athena-onboarding',
    //       name: 'pcc-athena-onboarding',
    //       component: Vue.component('pcc-athena-onboarding'),
    //     },
    //     {
    //       path: 'pcc-athena-progres-notes-page',
    //       name: 'pcc-athena-progres-notes-page',
    //       component: Vue.component('pcc-athena-progres-notes-page'),
    //     },
    //     {
    //       path: 'practitioners',
    //       name: 'practitioners-page',
    //       component: Vue.component('practitioners-page'),
    //     },
    //   ]
    // },
    {
      path: '/stock-builder',
      name: 'stock-builder',
      component: Vue.component('stock-builder-page'),
    },
  ]
});
var app = new Vue({ el: '#app', router: router });
