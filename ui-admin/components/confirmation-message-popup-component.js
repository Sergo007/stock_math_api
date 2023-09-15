Vue.component('confirmation-message-popup', {
  template: `
<popup-component 
 v-bind:popupId="popupId">
  <div class="app-popup" style="max-width: 500px; color: #868aa8">
    <span data-close class="close">×</span>
    <div class="app-popup-body">
      <div style="padding: 15px">
        <slot v-bind:item="openedWithValues"></slot>
      </div>
      <div class="confirmation-message-popup-buttons" style="display: flex">
        <button style="margin-right: 7px" class="button submit-button" v-on:click="clickYes()">YES <i v-if="isShowButtonSpinner" class="fa fa-spinner fa-spin"></i></button>
        <button style="margin-left: 7px" class="button submit-button" v-on:click="clickNo()">NO</button>
      </div>
    </div>
  </div> 
</popup-component>
  `,
  props: {
    id:{
      type: String,
      required: true
    },
    onClickYes: {
      type: Function,
      default: async function (values) {
        await new Promise(r => setTimeout(r, 2000));
      }
    },
    onClickNo: {
      type: Function,
      default: async function (values) {}
    }
  },
  data: function () {
    return {
      popupId: 'confirmation-message:' + this.id,
      isShowButtonSpinner: false,
      openedWithValues: {}
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
    clickYes: async function(){
      this.isShowButtonSpinner = true
      await this.onClickYes(this.openedWithValues);
      this.isShowButtonSpinner = false
      await new Promise(r => setTimeout(r, 100));
      this.close();
    },
    clickNo: function(){
      this.onClickNo(this.openedWithValues);
      this.close();
    },
    open: function(values) {
      $popupService.popup(this.popupId).open();
      this.openedWithValues = values;
    },
    close: async function() {
      $popupService.popup(this.popupId).close();
      await new Promise(r => setTimeout(r, 200));
      this.openedWithValues = {};
    }
  }
});
