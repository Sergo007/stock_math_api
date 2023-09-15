Vue.component('user-details', {
  template: `
  <div class="root-app-container">
    <div class="admin-header">
      <div class="trip-title">
        <h1>Guide details</h1>
      </div>
      <div class="admin-header-panel">
        <button v-on:click="layout()" class="layout_graph_button"></button>
      </div>
    </div>
    <div class="admin-container">
      <div class="admin-left-menu">
        <div class="admin-left-menu-controls">
          <div class="group">
            <h3 class="group-label">Basic actions</h3>
            <div class="group-content">
              <div v-on:click="addShellElement()" class="actions noselect shell-action"><span>Shell Action</span></div>
              <div class="actions noselect rest-api-action"><span>Rest Api Action</span></div>
              <div class="actions noselect rest-api-action"><span>trip Action</span></div>
            </div>
          </div>
        </div>
        <textarea id="adjacency-list" cols="35" rows="30"></textarea>
      </div>
      <div class="admin-pages-wrapper">
        
      </div>
      <div class="actions-form-wrapper">
        <div class="action-form-component">
          <div class="action-form-input">
            <span>title: </span>
            <input name="title" type="text" value="задача">
          </div>
          <div class="action-form-input">
            <span>description: </span>
            <textarea name="description"></textarea>
          </div>
          <div class="action-form-input">
            <span>settings1: </span>
            <input name="title" type="text" value="задача">
          </div>
          <div class="action-form-input">
            <span>settings2: </span>
            <select name="title" type="text" value="задача">
              <option>Пункт 1</option>
              <option>Пункт 2</option>
              <option>Пункт 3</option>
              <option>Пункт 4</option>
              <option>Пункт 5</option>
            </select>
          </div>
          <div class="action-form-input">
            <span>settings3: </span>
            <input name="title" type="text" value="задача">
          </div>
          <div class="action-form-input">
            <span>settings1: </span>
            <input name="title" type="text" value="задача">
          </div>
          <div class="action-form-input">
            <span>settings2: </span>
            <input name="title" type="text" value="задача">
          </div>
          <div class="action-form-input">
            <span>settings3: </span>
            <input name="title" type="text" value="задача">
          </div>
          <div class="action-form-input">
            <span>settings1: </span>
            <input name="title" type="text" value="задача">
          </div>
          <div class="action-form-input">
            <span>settings2: </span>
            <input name="title" type="text" value="задача">
          </div>
          <div class="action-form-input">
            <span>settings3: </span>
            <input name="title" type="text" value="задача">
          </div>
        </div>
      </div>
    </div>
    <terminal></terminal>
  </div>
  `,
  data: function () {
    return {
      graph: null,
      paper: null,
      isShowLogs: false
    }
  },
  methods: {
    myFunc: function () {

    }
  },
  mounted() {

  },
  beforeDestroy: function () {

  },
});
