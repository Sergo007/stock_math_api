'use strict';
Vue.component('admin', {
  template: `
  <div class="admin-page">
    <div class="admin-header">
      <div class="trip-title">
        <h1>Cybx admin</h1>
      </div>
      <div class="admin-header-panel">

      </div>
    </div>
    <div class="admin-container">
      <div class="admin-left-menu">
        <div class="menu-group">
          <router-link class="menu-item" style="display: none" v-bind:to="{ name: 'roles'}">roles</router-link>
          <router-link class="menu-item" style="display: none" v-bind:to="{ name: 'users'}">users</router-link>
          <router-link class="menu-item" v-bind:to="{ name: 'pcc-athena-onboarding'}">facilities onboarding</router-link>
          <router-link class="menu-item" v-bind:to="{ name: 'practitioners-page'}">practitioners</router-link>
          <router-link class="menu-item" v-bind:to="{ name: 'pcc-athena-progres-notes-page'}">pcc-->athena progres notes</router-link>
          <router-link class="menu-item" v-bind:to="{ name: 'progres-notes-search-page'}">progres notes search</router-link>
        </div>
      </div>
      <div class="admin-pages-wrapper">
        <router-view></router-view>
      </div>
      
    </div>
    <div class="admin-footer">
      <button class="button-logs">support</button>
      <button class="button-logs">contact us</button>
      <button class="button-logs">our team</button>
    </div>
    
  </div>
  `,
  mounted() {
    //$popupService.popup('create').open();
    let config = {
      headers: {
        'Authorization': 'Bearer ' + $storageService.get('auth-token'),
      }
    };
    // axios.post(window.URL + '/verify-token', this.form, config)
    //   .then( (response) => {
    //     var data = response.data;
    //     if(data.success) {

    //     } else {
    //       this.errorMessage = data.message;
    //     }
    //   })
    //   .catch( (error) => {
    //     this.$router.push('/login');
    //   })
  },
  methods: {

  },
  data: function () {
    return {
      isShowLogs: false
    }
  },
});
