<template>
  <div class="container">
    <h1>Zela Vue 3</h1>
    <button @click="callTest">Invoke Backend</button>
  </div>
</template>

<script setup>
import { invoke } from '@tauri-apps/api/tauri'

const callTest = () => {
  invoke('test').then(resp => {
    console.log(resp)
  })
}
</script>

<style>
.container {
  padding: 1rem;
}
</style>
