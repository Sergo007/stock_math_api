Vue.component('terminal', {
  template: `
  <div>
    <div class="terminal-wrapper">
      <div v-if="isShowLogs" class="logs">
        <p class="log-item">[trip name]: logs gfdsgfdsgf gfdsg fdgsfdsgf gfdsgfdsg fdgsfds gfdsgfdsg fdsgfdsgfdsg</p>
        <p class="log-item">[trip name]: logs gfdsgfdsgf gfdsg fdgsfdsgf gfdsgfdsg fdgsfds gfdsgfdsg fdsgfdsgfdsg</p>
        <p class="log-item">[trip name]: logs gfdsgfdsgf gfdsg fdgsfdsgf gfdsgfdsg fdgsfds gfdsgfdsg fdsgfdsgfdsg</p>
        <p class="log-item">[trip name]: logs gfdsgfdsgf gfdsg fdgsfdsgf gfdsgfdsg fdgsfds gfdsgfdsg fdsgfdsgfdsg</p>
        <p class="log-item">[trip name]: logs gfdsgfdsgf gfdsg fdgsfdsgf gfdsgfdsg fdgsfds gfdsgfdsg fdsgfdsgfdsg</p>
        <p class="log-item">[trip name]: logs gfdsgfdsgf gfdsg fdgsfdsgf gfdsgfdsg fdgsfds gfdsgfdsg fdsgfdsgfdsg</p>
        <p class="log-item">[trip name]: logs gfdsgfdsgf gfdsg fdgsfdsgf gfdsgfdsg fdgsfds gfdsgfdsg fdsgfdsgfdsg</p>
        <p class="log-item">[trip name]: logs gfdsgfdsgf gfdsg fdgsfdsgf gfdsgfdsg fdgsfds gfdsgfdsg fdsgfdsgfdsg</p>
        <p class="log-item">[trip name]: logs gfdsgfdsgf gfdsg fdgsfdsgf gfdsgfdsg fdgsfds gfdsgfdsg fdsgfdsgfdsg</p>
        <p class="log-item">[trip name]: logs gfdsgfdsgf gfdsg fdgsfdsgf gfdsgfdsg fdgsfds gfdsgfdsg fdsgfdsgfdsg</p>
        <p class="log-item">[trip name]: logs gfdsgfdsgf gfdsg fdgsfdsgf gfdsgfdsg fdgsfds gfdsgfdsg fdsgfdsgfdsg</p>
        <p class="log-item">[trip name]: logs gfdsgfdsgf gfdsg fdgsfdsgf gfdsgfdsg fdgsfds gfdsgfdsg fdsgfdsgfdsg</p>
        <p class="log-item">[trip name]: logs gfdsgfdsgf gfdsg fdgsfdsgf gfdsgfdsg fdgsfds gfdsgfdsg fdsgfdsgfdsg</p>
        <p class="log-item">[trip name]: logs gfdsgfdsgf gfdsg fdgsfdsgf gfdsgfdsg fdgsfds gfdsgfdsg fdsgfdsgfdsg</p>
        <p class="log-item">[trip name]: logs gfdsgfdsgf gfdsg fdgsfdsgf gfdsgfdsg fdgsfds gfdsgfdsg fdsgfdsgfdsg</p>
        <p class="log-item">[trip name]: logs gfdsgfdsgf gfdsg fdgsfdsgf gfdsgfdsg fdgsfds gfdsgfdsg fdsgfdsgfdsg</p>
        <p class="log-item">[trip name]: logs gfdsgfdsgf gfdsg fdgsfdsgf gfdsgfdsg fdgsfds gfdsgfdsg fdsgfdsgfdsg</p>
        <p class="log-item">[trip name]: logs gfdsgfdsgf gfdsg fdgsfdsgf gfdsgfdsg fdgsfds gfdsgfdsg fdsgfdsgfdsg</p>
        <p class="log-item">[trip name]: logs gfdsgfdsgf gfdsg fdgsfdsgf gfdsgfdsg fdgsfds gfdsgfdsg fdsgfdsgfdsg</p>
        <p class="log-item">[trip name]: logs gfdsgfdsgf gfdsg fdgsfdsgf gfdsgfdsg fdgsfds gfdsgfdsg fdsgfdsgfdsg</p>
        <p class="log-item">[trip name]: logs gfdsgfdsgf gfdsg fdgsfdsgf gfdsgfdsg fdgsfds gfdsgfdsg fdsgfdsgfdsg</p>
        <p class="log-item">[trip name]: logs gfdsgfdsgf gfdsg fdgsfdsgf gfdsgfdsg fdgsfds gfdsgfdsg fdsgfdsgfdsg</p>
        <p class="log-item">[trip name]: logs gfdsgfdsgf gfdsg fdgsfdsgf gfdsgfdsg fdgsfds gfdsgfdsg fdsgfdsgfdsg</p>
        <p class="log-item">[trip name]: logs gfdsgfdsgf gfdsg fdgsfdsgf gfdsgfdsg fdgsfds gfdsgfdsg fdsgfdsgfdsg</p>
        <p class="log-item">[trip name]: logs gfdsgfdsgf gfdsg fdgsfdsgf gfdsgfdsg fdgsfds gfdsgfdsg fdsgfdsgfdsg</p>
        <p class="log-item">[trip name]: logs gfdsgfdsgf gfdsg fdgsfdsgf gfdsgfdsg fdgsfds gfdsgfdsg fdsgfdsgfdsg</p>
        <p class="log-item">[trip name]: logs gfdsgfdsgf gfdsg fdgsfdsgf gfdsgfdsg fdgsfds gfdsgfdsg fdsgfdsgfdsg</p>
        <p class="log-item warn">[trip name]: warn</p>
        <p class="log-item error">[trip name]: error</p>
        <p class="log-item success">[trip name]: logs gfdsgfdsgf gfdsg fdgsfdsgf gfdsgfdsg fdgsfds gfdsgfdsg fdsgfdsgfdsg</p>
      </div>
    </div>
    <div class="admin-footer">
      <button v-on:click="isShowLogs = !isShowLogs" class="button-logs">logs</button>
    </div>
  </div>
  `,
  data: function () {
    return {
      isShowLogs: false
    }
  },
});
