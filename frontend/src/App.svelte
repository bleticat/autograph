<script>
  import { invoke } from "@tauri-apps/api/core";

  let projects = $state([]);
  let todos = $state([]);
  let selectedProjectId = $state(null); // null = "No Project" view
  let todoInput = $state("");
  let projectInput = $state("");

  async function loadProjects() {
    projects = await invoke("get_projects");
  }

  async function loadTodos() {
    if (selectedProjectId === null) {
      todos = await invoke("get_todos_without_project");
    } else {
      todos = await invoke("get_todos_by_project", { projectId: selectedProjectId });
    }
  }

  async function addTodo() {
    const title = todoInput.trim();
    if (!title) return;
    todoInput = "";
    await invoke("add_todo", {
      title,
      projectId: selectedProjectId,
    });
    await loadTodos();
  }

  async function toggleTodo(id) {
    await invoke("toggle_todo", { id });
    await loadTodos();
  }

  async function deleteTodo(id) {
    await invoke("delete_todo", { id });
    await loadTodos();
  }

  async function addProject() {
    const title = projectInput.trim();
    if (!title) return;
    projectInput = "";
    projects = await invoke("add_project", { title });
  }

  async function selectProject(id) {
    selectedProjectId = id;
    await loadTodos();
  }

  function handleTodoKeydown(e) {
    if (e.key === "Enter") addTodo();
  }

  function handleProjectKeydown(e) {
    if (e.key === "Enter") addProject();
  }

  $effect(() => {
    loadProjects();
    loadTodos();
  });
</script>

<div class="layout">
  <aside class="sidebar">
    <h2>Projects</h2>
    <ul class="project-list">
      <li
        class:active={selectedProjectId === null}
        onclick={() => selectProject(null)}
      >
        Inbox
      </li>
      {#each projects as project (project.id)}
        <li
          class:active={selectedProjectId === project.id}
          onclick={() => selectProject(project.id)}
        >
          {project.title}
        </li>
      {/each}
    </ul>
    <div class="project-input-row">
      <input
        type="text"
        placeholder="New project…"
        bind:value={projectInput}
        onkeydown={handleProjectKeydown}
      />
      <button onclick={addProject}>+</button>
    </div>
  </aside>

  <main>
    <h1>
      {selectedProjectId === null
        ? "Inbox"
        : projects.find((p) => p.id === selectedProjectId)?.title ?? "Inbox"}
    </h1>
    <div class="input-row">
      <input
        type="text"
        placeholder="What needs to be done?"
        bind:value={todoInput}
        onkeydown={handleTodoKeydown}
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
</div>
