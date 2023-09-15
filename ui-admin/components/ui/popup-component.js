this.$popupService = function () {
  'use strict';
  function PopupService() {
    this.popups = {};
  }
  PopupService.prototype.add = function (popupId, Popup) {
    this.popups[popupId] = Popup;
  };
  PopupService.prototype.remove = function (id) {
    this.popups[id] = undefined;
  };
  PopupService.prototype.popup = function (id) {
    return this.popups[id];
  };
  PopupService.prototype.getPopupComponent = function (id) {
    return this.popups[id]._component;
  };
  PopupService.prototype.registerPopupComponent = function (id, component) {
    if (this.popups[id] !== undefined) {
      this.popups[id]._component = component;
    }
    else {
      console.error('Please use registerPopupComponent("' + id + '", this) in  ngAfterViewInit() method');
    }
  };
  return new PopupService();
}();

Vue.component('popup-component', {
  template: `
<div class="smart-popup-template" style="display: none;" v-bind:style="{ wordSpacing: '-'+fixHorizontalPosition+'em' }" v-on:click="clickPopup">
  <div v-if="backgroundEnable" data-close-inner class="bg" v-bind:style="{ backgroundColor: theBackground }"></div>
  <div data-close-inner class="popup-body">
    <div data-close-inner class="popup-content">
      <slot></slot>
    </div>
  </div>
</div>
  `,
  props: {
    popupId: {
      type: String,
      required: true
    },
    theBackground: {
      type: String,
      default: 'rgba(255, 255, 255, 0.8)'
    },
    fadeTime: {
      type: Number,
      default: 200
    },
    fixHorizontalPosition: {
      type: Number,
      default: 0.3
    },
    widthScrollBar: {
      type: Number,
      default: 0
    },
    backgroundEnable: {
      type: Boolean,
      default: true
    }
  },
  data: function () {
    return {
      isShowLogs: false,
      isOpened: false
    }
  },
  created() {
    this.popupLayout = {};
    this.body = {};
    this.isMobile = {
      Android: function () {
        return navigator.userAgent.match(/Android/i) ? true : false;
      },
      BlackBerry: function () {
        return navigator.userAgent.match(/BlackBerry/i) ? true : false;
      },
      iOS: function () {
        return navigator.userAgent.match(/iPhone|iPad|iPod/i) ? true : false;
      },
      Windows: function () {
        return navigator.userAgent.match(/IEMobile/i) ? true : false;
      },
      any: () => {
        return (this.isMobile.Android() || this.isMobile.BlackBerry() || this.isMobile.iOS() || this.isMobile.Windows());
      }
    };
  },
  mounted() {
    this.popupLayout = this.$el;
    this.html = document.getElementsByTagName('html')[0];
    this.body = document.getElementsByTagName('body')[0];
    $popupService.add(this.popupId, this);
  },
  beforeDestroy: function () {
    this.close();
    $popupService.remove(this.popupId);
  },
  methods: {
    clickPopup: function(event) {
      let e = event.target;
      if (e.hasAttribute('data-close')) {
        this.close();
      }
      if (e.hasAttribute('data-close-inner')) {
        if (this.backgroundEnable) {
          // this.close();
        }
      }
    },
    open: function() {
      this.html.style.overtrip = 'hidden';
      this.body.style.overtrip = 'hidden';
      if (this.isMobile.iOS()) {
        this.body.style.position = 'fixed';
      }
      if (!this.isMobile.any()) {
        this.body.style.paddingRight = this.widthScrollBar + 'px';
      }
      this.popupLayout.style.opacity = '0';
      this.popupLayout.style.position = 'fixed';
      this.popupLayout.style.transition = 'opacity ' + this.fadeTime * 0.001 + 's ease-in-out';
      this.popupLayout.style.display = null;
      setTimeout(() => {
        this.popupLayout.style.opacity = '1';
        this.$emit('onOpenEvent', null)
        this.isOpened = true;
      });
    },

    close: function() {
      this.popupLayout.style.opacity = '0';
      setTimeout(() => {
        this.isOpened = false;
        this.$emit('onCloseEvent', null)
        this.popupLayout.style.display = 'none';
        this.html.style.overtrip = null;
        this.body.style.overtrip = null;
        this.body.style.position = null;
        if (!this.isMobile.any()) {
          this.body.style.paddingRight = null;
        }
      }, this.fadeTime);
    }
  },
});
