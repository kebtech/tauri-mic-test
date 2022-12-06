<template>
  <div class="container">
    <button @click="tryStream">Try stream and show devices</button>
    <div class="stream-response">{{status? 'streaming from ' + device: 'not streaming'}}</div>
    <div class="device" v-for="device in devices" :key="device.deviceId">{{device.deviceId + ' - ' + device.label}}</div>
  </div>
</template>

<script lang="ts">
import { defineComponent, reactive, ref } from 'vue';

export default defineComponent({
  setup() {
    const devices: MediaDeviceInfo[] = reactive([]);
    const status = ref(false);
    const device = ref('')

    const showDevices = () => {
      devices.length = 0;
      navigator.mediaDevices.enumerateDevices()
        .then(r => {
          r.forEach(d => devices.push(d))
        })
    }

    const tryStream = () => {
      navigator.mediaDevices.getUserMedia({audio: true, video: false})
        .then(s => {
          device.value = s.getTracks()[0].label;
          status.value = s.active;
          showDevices();
          setTimeout(() => {
            const tracks = s.getTracks() || [];
            
            tracks.forEach(function(track) {
                track.stop();
            });
            status.value = false;
            showDevices()
          }, 5000 )
        })
      
    }

    return { tryStream, devices, status, device }
  }
});
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
