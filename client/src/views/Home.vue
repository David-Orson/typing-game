<script setup lang="ts">
// npm
import { ref, onMounted } from "vue";

// services
import { useServices } from "@/data/services";
const { finishTest } = useServices();

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

const sessionPr = ref(0);

const wordlist =
  "anything then the best world will defeat any bother and I have any great interest ground govern anytime major sensitive west time gripe nose big elephant has always been able to upset the small one";

const underscores = "____________________________";

// watchers
window.addEventListener("keydown", (e) => {
  if (!isTestActive.value && timeLeft.value === 15) reset();

  let allKeys = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

  allKeys.split("").forEach((c) => {
    if (e.key === c) typed.value += c;
  });

  if (e.key === " ") typed.value += " ";

  if (e.key === "Backspace") typed.value = typed.value.slice(0, -1);

  if (e.key === "Enter") {
    newTest();
    populateTestString();
  }
});

// methods
const reset = () => {
  isTestActive.value = true;

  intervalId.value = setInterval(() => {
    timeLeft.value -= 1;

    // calc wpm
    wpm.value = Math.round(
      ((typed.value.length / 5) * 60) / (timeTotal.value - timeLeft.value)
    );

    // endgame
    if (timeLeft.value === 0) {
      isTestActive.value = false;

      clearInterval(intervalId.value);

      if (accuracy.value === 100 && wpm.value > sessionPr.value)
        sessionPr.value = wpm.value;

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
});
</script>

<template>
  <div class="container mx-auto">
    <div>{{ timeLeft }} {{ wpm }} {{ accuracy }} PR: {{ sessionPr }}</div>

    <div class="mx-16 text-4xl text-gray-700">{{ test }}</div>

    <div class="m-16 text-4xl flex justify-center flex-wrap">
      <div class="flex" v-for="(word, i) in typed.split(' ')">
        <div v-for="(c, j) in word.split('')">
          <span
            class="w-1"
            :class="
              test.split(' ')[i] &&
              test.split(' ')[i][j] &&
              test.split(' ')[i][j] == typed.split(' ')[i][j]
                ? 'text-gray-700'
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
