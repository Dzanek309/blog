<script setup>
import { ref } from 'vue';
import { my_project_backend } from 'declarations/my_project_backend/index';
let greeting = ref('');

async function handleSubmit(e) {
  e.preventDefault();
  const target = e.target;
  const title = targe.querySelector('#title').value;
  const tags = target.querySelector('#tags').value;
  const splitedTags = tags.split(',')

  await my_project_backend.add_blog(title, content, splitedTags)
}
</script>

<template>
  <main>
    <!-- <img src="/logo2.svg" alt="DFINITY logo" /> -->
    <br />
    <br />
    <form action="#" @submit="handleSubmit">
      <div>
        <p>Title:</p><input id="title" type="text" alt="title">
      </div>
      <div>
        <p>Content:</p><input id="content" type="text" alt="title">
      </div>
      <div>
        <p>Tags:</p><input id="tags" type="text" alt="title">
      </div>
      <button type="submit">Click Me!</button>
    </form>
    <section id="greeting">{{ greeting }}</section>
  </main>
</template>
