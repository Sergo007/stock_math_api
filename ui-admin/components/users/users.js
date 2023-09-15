'use strict';

Vue.component('users', {
  template: `
  <div class="pcc-athena-onboarding-page">  
    <div class="trips">
      <div class="buttons">
        <button class="button" v-on:click="sendForm()">create user</button>
      </div>
    <Table class="flexy" style="height: calc(100% - 106px)">
      <TableHead>
        <TableRow inHead>
          <TableCell inHead scope="col">name</TableCell>
          <TableCell inHead scope="col">email</TableCell>
          <TableCell inHead scope="col">phone</TableCell>
        </TableRow>
      </TableHead>
      <TableBody>
        <TableRow v-for="user in users" :key="user.user_id">
          <TableCell scope="row">
            {{user.user_name}}
          </TableCell>
          <TableCell>
            {{user.email}}
          </TableCell>
          <TableCell >
            {{user.phone}}
          </TableCell>
        </TableRow>
      </TableBody>
    </Table>
 
   </div>
   <new-user-popup></new-user-popup>
  </div>
  `,
  mounted() {
    // $popupService.getPopupComponent('create-new-guide').open();
    let config = {
      headers: {
        'Authorization': 'Bearer ' + $storageService.get('auth-token'),
      }
    };
    axios.get('/api/user', config)
      .then( (response) => {
        var data = response.data;
        if(data.status==="success") {
          this.users = data.data;
          console.log(data.data);
        }
      })
      .catch( (error) => {
        console.log(error)
      })
  },
  methods: {
    toDetainstrip: function(tripId) {
      this.$router.push('/guide/' + tripId);
    },
    createProject: function(tripId) {
      this.$router.push('/guide/' + tripId);
    },
    sendForm: function () {
      $popupService.getPopupComponent('create-new-user').open();
    }
  },
  data: function () {
    return {
      isShowLogs: false,
      users: []
    }
  },
});
