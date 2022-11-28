<script setup lang="ts">
// npm
import { ref, onMounted, onUnmounted } from "vue";

// services
import { useServices } from "@/data/services";

// hooks
import { useStore } from "@/store";
import { useLocalState } from "@/hooks/localStateHook";

const { finishTest, onTestEnd, onTestStart } = useServices();
const { addState } = useLocalState();

const store = useStore();

// reactive
const isTestActive = ref(false);
const wpm = ref();
const test = ref("");
const typed = ref("");
const timeLeft = ref(15);
const timeTotal = ref(15);
const intervalId = ref();
const accuracy = ref(100);
const errors = ref(0);
const spaceErrors = ref(0);

const wordlist =
  "of off and a to two too in is you that it he for four was on are as with his they at be bee this from I eye have or ore by bye buy one won had not knot but what all were where when we there their they're can an your you're which witch said if do due will each about how who up out them then than she many some sum so sew these would wood other into has more her like him see sea time could no know make first been its it’s now people my made maid over did down done only way weigh find fined use used may water long little very after words called just most get through back much before go good new knew write right our hour me man men woman women any day same look think also around another came come work three word must because does part even place well such here hear take why things help put years different away again went old number great tell say small every found still between name should Mr Mrs Ms Miss home big give air line set own under read red last never us left end along while might next sound below saw something thought both few those always looked show large often together asked house don't world going want";

const underscores = "____________________________";

// watchers
const keyListener = (e: any) => {
  if (e.key === "Enter") {
    newTest();
    populateTestString();
  } else if (!typed.value.length && !isTestActive.value) reset();

  if (isTestActive.value) {
    let allKeys =
      "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890[];'#/.,!£$%^&*()_+=-{}@:~?><";

    allKeys.split("").forEach((c) => {
      if (e.key === c) typed.value += c;
    });

    if (e.key === " ") typed.value += " ";

    if (e.key === "Backspace") typed.value = typed.value.slice(0, -1);
  }
};

// methods
const reset = () => {
  isTestActive.value = true;
  if (store.state.environment === "production") {
    onTestStart();
  }
  clearInterval(intervalId.value);

  intervalId.value = setInterval(() => {
    timeLeft.value -= 1;

    // calc wpm
    wpm.value = Math.round(
      ((typed.value.length / 5) * 60) / (timeTotal.value - timeLeft.value)
    );

    // endgame
    if (timeLeft.value === 0) {
      isTestActive.value = false;

      if (store.state.environment === "production") {
        onTestEnd();
      }

      clearInterval(intervalId.value);

      if (
        (store.state.sessionPr &&
          wpm.value > store.state.sessionPr &&
          accuracy.value === 100) ||
        (!store.state.sessionPr && accuracy.value === 100)
      )
        addState("sessionPr", wpm.value);

      finishTest(test.value, typed.value);
    }

    // reset errors
    errors.value = 0;
    spaceErrors.value = 0;

    const typedSplit = typed.value.split(" ");
    const testSplit = test.value.split(" ");

    // count errors
    typedSplit.forEach((word, i) => {
      word.split("").forEach((c, j) => {
        if (c != testSplit[i][j]) {
          errors.value++;
        }
      });

      if (i !== typedSplit.length - 1) {
        errors.value += testSplit[i].length - word.length;
        spaceErrors.value += testSplit[i].length - word.length;
      }
    });

    // calculate accuracy
    if (typed.value.length)
      accuracy.value = Math.round(
        100 -
          (errors.value /
            (typed.value.replace(" ", "").length + spaceErrors.value)) *
            100
      );
  }, 1000);
};

const populateTestString = () => {
  const arr = wordlist.split(" ");
  for (let i = 0; i < 35; i++) {
    if (i !== 0) {
      test.value += " ";
    }

    test.value += arr[Math.floor(Math.random() * arr.length)];
  }
};

const newTest = () => {
  clearInterval(intervalId.value);
  isTestActive.value = false;

  // reset test values
  timeLeft.value = 15;
  accuracy.value = 100;
  test.value = "";
  typed.value = "";
};

// lifecycle
onMounted(() => {
  populateTestString();
  window.addEventListener("keydown", keyListener);
});

onUnmounted(() => {
  clearInterval(intervalId.value);
  window.removeEventListener("keydown", keyListener);
});
</script>

<template>
  <div class="container mx-auto">
    <div class="mb-4">
      Time: {{ timeLeft }} | WPM: {{ wpm }} | Accuracy: {{ accuracy }} | Session
      PR: {{ store.state.sessionPr || 0 }} | PR:
      {{ store.getters.account?.pr || "Login to save Personal Record" }}
    </div>

    <div class="green-text">Test Text:</div>
    <div class="mx-16 text-4xl mb-4">{{ test }}</div>

    <div class="green-text">Typed Text:</div>
    <div class="mx-16 text-4xl flex justify-center flex-wrap">
      <div class="flex" v-for="(word, i) in typed.split(' ')">
        <div v-for="(c, j) in word.split('')">
          <span
            class="w-1"
            :class="
              test.split(' ')[i] &&
              test.split(' ')[i][j] &&
              test.split(' ')[i][j] == typed.split(' ')[i][j]
                ? 'text-white-700'
                : 'text-red-500'
            "
            >{{ c }}</span
          >
        </div>
        <div
          v-if="
            typed.split(' ').length > i + 1 &&
            word.length !== test.split(' ')[i].length
          "
          class="text-red-500"
        >
          {{
            test.split(" ")[i].length - word.length > 0
              ? underscores.slice(0, test.split(" ")[i].length - word.length)
              : ""
          }}
        </div>
        <div class="w-2"></div>
      </div>
    </div>
  </div>
</template>
