/**
 * example for use
 <popup-form-repeater
 v-bind:formData="form.phoneNumbers"
 v-bind:formDataItem="{phone: ''}"
 v-bind:headerText="'Phone numbers:'"
 v-bind:buttonAddText="'add phone'"
 v-slot:repeat="{item: item}">
   <span>phone:</span>
   <input  type="text" value="" v-model="item.phone">
 </popup-form-repeater>
 */

Vue.component('popup-form-repeater', {
  template: `
<div class="repeater-form">
  <span class="repeater-form-header">{{headerText}}</span>
  <div class="repeater-form-item" v-for="(v, k, i) in formData">
    <div class="repeater-form-inputs-item">
      <slot name="repeat" v-bind:item="v"></slot>
    </div>
    <div class="repeater-form-item-remove noselect" 
    v-if="formData.length > 1" v-on:click="formData.splice(i,1)">×</div>
  </div>
  <div class="button repeater-form-add noselect" v-on:click="add()">{{buttonAddText}}</div>
</div>
  `,
  props: {
    formData: {
      type: Array,
      required: true
    },
    formDataItem: {
      type: Object,
      required: true
    },
    headerText: {
      type: String,
      required: true
    },
    buttonAddText: {
      type: String,
      required: true
    }
  },
  data: function () {
    return {}
  },
  created() {

  },
  mounted() {

  },
  beforeDestroy: function () {

  },
  methods: {
    add: function () {
      this.formData.push(this.formDataItem);
    },
  },
});
