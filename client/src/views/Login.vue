<script setup lang="ts">
import { ref } from "vue";

// primevue
import InputText from "primevue/inputtext";
import Password from "primevue/password";
import Button from "primevue/button";

// reactive
const creds = ref<any>({});

// methods
const logInCallback = async () => {};
</script>

<template>
  <div class="flex justify-center">
    <form
      class="flex flex-col justify-evenly h-40"
      @submit.prevent="logInCallback"
    >
      <InputText id="email-input" v-model="creds.email" placeholder="Email" />
      <Password
        :feedback="false"
        v-model="creds.password"
        placeholder="Password"
      />
      <Button class="mt-4" label="Log In" type="submit" />
    </form>
    <div class="w-16"></div>
    <form
      class="flex flex-col justify-evenly h-64"
      @submit.prevent="logInCallback"
    >
      <InputText
        id="username-input"
        v-model="creds.name"
        placeholder="Username"
      />
      <InputText id="email-input" v-model="creds.email" placeholder="Email" />
      <Password :feedback="true" v-model="creds.pass" placeholder="Password" />
      <Password
        :feedback="true"
        v-model="creds.pass2"
        placeholder="Confirm Password"
      />
      <Button class="mt-4" label="Sign Up" type="submit" />
    </form>
  </div>
</template>
