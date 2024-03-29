Vue.component('new-project-popup', {
  template: `
<popup-component 
 v-bind:popupId="popupId">
  <div class="app-popup">
    <span data-close class="close">×</span>
    <div class="app-popup-body">
      <h3>Create new project</h3>
      <div class="action-form-component">
        <div class="action-form-input">
          <span>project name: </span>
          <input ref="firstInput" name="title" type="text" v-model="form.name">
        </div>
        <div class="action-form-input">
          <span>project description: </span>
          <textarea name="description" v-model="form.description"></textarea>
        </div>
      </div>
      <button class="button submit-button" v-on:click="create(path)">Create project</button>
    </div>
  </div> 
</popup-component>
  `,
  props: {
    createProject: {
      type: Function,
      default: function (path, dirName) {}
    }
  },
  data: function () {
    return {
      popupId: 'new-project',
      form: {
        name: '',
        description: '',
      },
      path: ''
    }
  },
  created() {

  },
  mounted() {
    $popupService.registerPopupComponent(this.popupId,this);
  },
  beforeDestroy: function () {

  },
  methods: {
    create: function(path){
      this.createProject(path, {
        name: this.form.name,
        description: this.form.description,
      });
      this.close();
    },
    open: function(path) {
      $popupService.popup(this.popupId).open();
      this.$refs.firstInput.focus();
      this.path = path;
    },

    close: function() {
      $popupService.popup(this.popupId).close();
      this.path = '';
      this.form.name = '';
      this.form.description = '';
    }
  }
});
