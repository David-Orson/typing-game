<script setup lang="ts">
import { ref } from "vue";

// primevue
import InputText from "primevue/inputtext";
import Password from "primevue/password";
import Button from "primevue/button";

// models
import { LoginCreds, SignupCreds } from "@/data/models";

// services
import { useServices } from "@/data/services";
const { logIn, signUp } = useServices();

// reactive
const loginCreds = ref<LoginCreds>({} as LoginCreds);
const signupCreds = ref<SignupCreds>({} as SignupCreds);
const confirmPass = ref("");

// methods
const logInCallback = async () => {
  logIn(loginCreds.value);
};

const signUpCallback = async () => {
  signUp(signupCreds.value);
};
</script>

<template>
  <div class="flex justify-center">
    <form
      class="flex flex-col justify-evenly h-40"
      @submit.prevent="logInCallback"
    >
      <InputText
        id="email-input"
        v-model="loginCreds.email"
        placeholder="Email"
      />
      <Password
        :feedback="false"
        v-model="loginCreds.password"
        placeholder="Password"
      />
      <Button class="mt-4" label="Log In" type="submit" />
    </form>
    <div class="w-16"></div>
    <form
      class="flex flex-col justify-evenly h-64"
      @submit.prevent="signUpCallback"
    >
      <InputText
        id="username-input"
        v-model="signupCreds.username"
        placeholder="Username"
      />
      <InputText
        id="email-input"
        v-model="signupCreds.email"
        placeholder="Email"
      />
      <Password
        :feedback="true"
        v-model="signupCreds.password"
        placeholder="Password"
      />
      <Password
        :feedback="true"
        v-model="confirmPass"
        placeholder="Confirm Password"
      />
      <Button class="mt-4" label="Sign Up" type="submit" />
    </form>
  </div>
</template>
