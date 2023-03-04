<script setup lang="ts">
// npm
import { onMounted } from "vue";

// hooks
import { useStore } from "@/store";
import { useServices } from "@/data/services";
const { onVisit } = useServices();
const store = useStore();

const genRanHex = () =>
  [...Array(100)]
    .map(() => Math.floor(Math.random() * 16).toString(16))
    .join("");

onMounted(async () => {
  if (!store.state.sessionToken) {
    const token = genRanHex();
    store.state.sessionToken = token;
    localStorage.setItem("sessionToken", token);
  }

  if (store.state.environment === "production") {
    await onVisit();
  }
});
</script>

<template>
  <nav>
    <router-link to="/">Home</router-link> |
    <router-link to="/about">About</router-link> |
    <router-link v-if="!store.state.account" to="/login">Login</router-link>
    <router-link v-if="store.state.account" to="/account">{{
      store.getters.account.username
    }}</router-link>
  </nav>
  <router-view />
</template>

<style>
@import "./index.css";

#app {
  font-family: "Lucida Sans", "Lucida Sans Regular", "Lucida Grande",
    "Lucida Sans Unicode", Geneva, Verdana, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #4b5975;
  background-color: #1b2028;
  min-height: 100vh;
}

.typed {
  color: #ccccb5;
}

nav {
  padding: 30px;
}

nav a {
  font-weight: bold;
  color: #e6e6e6;
}

nav a.router-link-exact-active {
  color: #42b983;
}
</style>
