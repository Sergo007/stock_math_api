Vue.component('edit-geometry-item-popup', {
  template: `
<popup-component v-bind:popupId="popupId">
  <div class="app-popup" style="max-width: 600px; margin-top: 15px; margin-bottom: 15px;">
    <span data-close class="close">×</span>
    <div class="app-popup-body">
      <h3>Edit stock geometry item</h3>
      <div class="action-form-component">
        <div class="action-form-input">
          <span>Cell type: * </span>
          <select ref="firstInput" v-model="form.type">
            <option value="empty">empty</option>
            <option value="stock_item">stock item</option>
            <option value="big_stock_item">big stock item</option>
            <option value="packaging_point">packaging point</option>
            <option value="wall">wall</option>
          </select>
          <div v-if="form.type=='stock_item'" class="form-col-12">          
            <span>Блок стеллажей №: </span>
            <input  type="text" value="" v-model="form.stock_item.rowId">
            <span>Стеллаж №: </span>
            <input  type="text" value="" v-model="form.stock_item.blockItemId">
          </div>
        </div>
      </div>
      <button class="button submit-button" v-on:click="submit()">submit <i v-if="isShowButtonSpinner" class="fa fa-spinner fa-spin"></i></button>
    </div>
  </div>
</popup-component>
   `,
  props: {
    submitButtonEvent: {
      type: Function,
      default: async function (item, x, y) {
        await new Promise(r => setTimeout(r, 2000));
      }
    }
  },
  data: function () {
    return {
      popupId: 'edit-geometry-item-popup',
      isShowButtonSpinner: false,
      x: 0,
      y: 0,
      form: {
        type: "",
        stock_item: {
          rowId: "",
          blockItemId: ""
        },
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
    submit: async function () {
      this.isShowButtonSpinner = true
      let item = JSON.parse(JSON.stringify(this.form))
      if (item.type != "stock_item") {
        delete item.stock_item
      }
      await this.submitButtonEvent(item ,this.x, this.y)
      this.isShowButtonSpinner = false
      await new Promise(r => setTimeout(r, 100));
      this.close()
    },
    open: function (item,x,y) {
      // console.log("open",JSON.stringify(item,null,2))
      if (item.stock_item == "") {
        delete item.stock_item
      }
      $popupService.popup(this.popupId).open();
      this.x = x
      this.y = y
      for (let key in item) {
        this.form[key] = item[key];
      }
      this.$refs.firstInput.focus();
    },

    close: function () {
      $popupService.popup(this.popupId).close();
      // console.log("open",JSON.stringify(this.form,null,2))
      // for (let key in this.form) {
      //   if (key !=  'stock_item'){
      //     this.form[key] = '';
      //   }
      // }
    }
  }
});
