<script>
  import { invoke } from "@tauri-apps/api/core";

  let todos = $state([]);
  let inputValue = $state("");

  async function loadTodos() {
    todos = await invoke("get_todos");
  }

  async function addTodo() {
    const title = inputValue.trim();
    if (!title) return;
    inputValue = "";
    todos = await invoke("add_todo", { title });
  }

  async function toggleTodo(id) {
    todos = await invoke("toggle_todo", { id });
  }

  async function deleteTodo(id) {
    todos = await invoke("delete_todo", { id });
  }

  function handleKeydown(e) {
    if (e.key === "Enter") addTodo();
  }

  $effect(() => {
    loadTodos();
  });
</script>

<main>
  <h1>Autograph Todo</h1>
  <div class="input-row">
    <input
      type="text"
      placeholder="What needs to be done?"
      bind:value={inputValue}
      onkeydown={handleKeydown}
    />
    <button onclick={addTodo}>Add</button>
  </div>
  <ul class="todo-list">
    {#each todos as todo (todo.id)}
      <li class:completed={todo.completed}>
        <input
          type="checkbox"
          checked={todo.completed}
          onchange={() => toggleTodo(todo.id)}
        />
        <span>{todo.title}</span>
        <button class="delete" onclick={() => deleteTodo(todo.id)}>&times;</button>
      </li>
    {/each}
  </ul>
</main>
