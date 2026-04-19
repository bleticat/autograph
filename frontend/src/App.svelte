<script>
  import { invoke } from "@tauri-apps/api/core";

  let projects = $state([]);
  let todos = $state([]);
  let selectedProjectId = $state(null); // null = "No Project" view
  let todoInput = $state("");
  let projectInput = $state("");
  let editingTodoId = $state(null);
  let editTitle = $state("");
  let editDescription = $state("");
  let editDeadline = $state("");
  const editingTodo = $derived(todos.find((todo) => todo.id === editingTodoId) ?? null);

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

  function openTodoEditor(todo) {
    editingTodoId = todo.id;
    editTitle = todo.title;
    editDescription = todo.description ?? "";
    editDeadline = todo.deadline ? todo.deadline.slice(0, 10) : "";
  }

  function closeTodoEditor() {
    editingTodoId = null;
    editTitle = "";
    editDescription = "";
    editDeadline = "";
  }

  async function saveTodoEdits() {
    const title = editTitle.trim();
    if (!editingTodoId || !title) return;
    await invoke("update_todo", {
      id: editingTodoId,
      title,
      description: editDescription.trim(),
      deadline: editDeadline || null,
    });
    await loadTodos();
    closeTodoEditor();
  }

  async function addProject() {
    const title = projectInput.trim();
    if (!title) return;
    projectInput = "";
    projects = await invoke("add_project", { title });
  }

  function selectProject(id) {
    selectedProjectId = id;
    closeTodoEditor();
  }

  function handleTodoKeydown(e) {
    if (e.key === "Enter") addTodo();
  }

  function handleProjectKeydown(e) {
    if (e.key === "Enter") addProject();
  }

  $effect(() => {
    loadProjects();
  });

  $effect(() => {
    loadTodos();
  });

  $effect(() => {
    if (editingTodoId !== null && !todos.some((todo) => todo.id === editingTodoId)) {
      closeTodoEditor();
    }
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
    {#if editingTodo}
      <section class="edit-page">
        <h2>Edit task</h2>
        <label>
          Title
          <input type="text" bind:value={editTitle} />
        </label>
        <label>
          Description
          <textarea rows="4" bind:value={editDescription}></textarea>
        </label>
        <label>
          Deadline
          <input type="date" bind:value={editDeadline} />
        </label>
        <div class="edit-actions">
          <button onclick={saveTodoEdits}>Save</button>
          <button class="secondary" onclick={closeTodoEditor}>Back</button>
        </div>
      </section>
    {:else}
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
            <button class="edit" onclick={() => openTodoEditor(todo)}>Edit</button>
            <button class="delete" onclick={() => deleteTodo(todo.id)}>&times;</button>
          </li>
        {/each}
      </ul>
    {/if}
  </main>
</div>
