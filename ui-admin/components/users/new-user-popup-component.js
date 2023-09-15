Vue.component('new-user-popup', {
  template: `
<popup-component 
 v-bind:popupId="popupId">
  <div class="app-popup" style="max-width: 600px; margin-top: 15px; margin-bottom: 15px;">
    <span data-close class="close">×</span>
    <div class="app-popup-body">
      <h3>Create new guide</h3>
      <div class="action-form-component">
        <div class="action-form-input">
          <div class="form-col-6">
            <span>first name: </span>
            <input ref="firstInput" type="text" value="" v-model="form.firstName">
          </div>
          <div class="form-col-6">          
            <span>last name: </span>
            <input  type="text" value="" v-model="form.lastName">
          </div>
          
          <popup-form-repeater 
          v-bind:formData="form.phoneNumbers"
          v-bind:formDataItem="{phone: '', description: ''}"
          v-bind:headerText="'Phone numbers:'"
          v-bind:buttonAddText="'add phone'"
          v-slot:repeat="{item: item}">
            <span>phone:</span>
            <input  type="text" value="" v-model="item.phone">
            <span>description:</span>
            <input  type="text" value="" v-model="item.description">
          </popup-form-repeater>
          
          <popup-form-repeater 
          v-bind:formData="form.emails"
          v-bind:formDataItem="{email: ''}"
          v-bind:headerText="'emails:'"
          v-bind:buttonAddText="'add email'"
          v-slot:repeat="{item: item}">
            <span>email:</span>
            <input  type="text" value="" v-model="item.email">
          </popup-form-repeater>
          
          <popup-form-repeater 
          v-bind:formData="form.messengers"
          v-bind:formDataItem="{messenger: '', messengerContact: ''}"
          v-bind:headerText="'messengers:'"
          v-bind:buttonAddText="'add messenger'"
          v-slot:repeat="{item: item}">
            <span>messenger:</span>
            <input  type="text" value="" v-model="item.messenger">
            <span>(nickname or phone number for messenger)</span>
            <input  type="text" value="" v-model="item.messengerContact">
          </popup-form-repeater>
          
          <span>photos: (возможность добавить больше чем 1)</span>
          <input  type="text" value="" v-model="form.photos">
          
          <popup-form-repeater 
          v-bind:formData="form.languages"
          v-bind:formDataItem="{language: ''}"
          v-bind:headerText="'languages:'"
          v-bind:buttonAddText="'add language'"
          v-slot:repeat="{item: item}">
            <span>language:</span>
            <select name="title" value="" v-model="item.language" style="height: 30px;">
              <option>ru</option>
              <option>en</option>
              <option>ge</option>
            </select>
          </popup-form-repeater>
          
          <span>price: </span>
          <input  type="text" value="" v-model="form.price">
          <span>description: </span>
          <textarea type="text" v-model="form.description"></textarea>
        </div>
      </div>
      <button class="button submit-button" v-on:click="create()">Create guide</button>
    </div>
  </div> 
</popup-component>
  `,
  props: {
    createdTrip: {
      type: Function,
      default: function (response, error) {
      }
    }
  },
  data: function () {
    return {
      popupId: 'create-new-user',
      form: {
        firstName: '',
        lastName: '',
        description: '',
        emails: [{email: ''}],
        messengers: [{messenger: '', messengerContact: ''}],
        languages: [{phone: '', description: ''}],
        price: '',
        photos: '',
        phoneNumbers: [{phone: ''}]
      },
    }
  },
  created() {

  },
  mounted() {
    $popupService.registerPopupComponent(this.popupId, this);
  },
  beforeDestroy: function () {

  },
  methods: {
    loging: function (log) {
      console.log(log);
    },
    create: function () {
      console.log(JSON.stringify(this.form));
    },
    open: function () {
      $popupService.popup(this.popupId).open();
      this.$refs.firstInput.focus();

    },

    close: function () {
      $popupService.popup(this.popupId).close();
      for (let key in this.form) {
        this.form[key] = '';
      }
    }
  }
});
