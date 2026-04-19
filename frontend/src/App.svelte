<script>
  import { invoke } from "@tauri-apps/api/core";

  let projects = $state([]);
  let todos = $state([]);
  let events = $state([]);
  let selectedProjectId = $state(null); // null = "No Project" view
  let selectedTab = $state("todos");
  let todoInput = $state("");
  let eventTitleInput = $state("");
  let eventDateInput = $state("");
  let projectInput = $state("");
  let editingTodoId = $state(null);
  let editingEventId = $state(null);
  let editTitle = $state("");
  let editDescription = $state("");
  let editDeadline = $state("");
  let editEventDate = $state("");
  const editingTodo = $derived(todos.find((todo) => todo.id === editingTodoId) ?? null);
  const editingEvent = $derived(events.find((event) => event.id === editingEventId) ?? null);

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

  async function loadEvents() {
    if (selectedProjectId === null) {
      events = await invoke("get_events_without_project");
    } else {
      events = await invoke("get_events_by_project", { projectId: selectedProjectId });
    }
  }

  async function addEvent() {
    const title = eventTitleInput.trim();
    const date = eventDateInput;
    if (!title || !date) return;
    eventTitleInput = "";
    eventDateInput = "";
    await invoke("add_event", {
      title,
      date,
      description: "",
      projectId: selectedProjectId,
    });
    await loadEvents();
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
    editingEventId = null;
    editTitle = todo.title;
    editDescription = todo.description ?? "";
    editDeadline = todo.deadline ? todo.deadline.slice(0, 10) : "";
    editEventDate = "";
  }

  function openEventEditor(event) {
    editingEventId = event.id;
    editingTodoId = null;
    editTitle = event.title;
    editDescription = event.description ?? "";
    editEventDate = event.date ? event.date.slice(0, 10) : "";
    editDeadline = "";
  }

  function closeEditor() {
    editingTodoId = null;
    editingEventId = null;
    editTitle = "";
    editDescription = "";
    editDeadline = "";
    editEventDate = "";
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
    closeEditor();
  }

  async function saveEventEdits() {
    const title = editTitle.trim();
    if (!editingEventId || !title || !editEventDate) return;
    await invoke("update_event", {
      id: editingEventId,
      title,
      description: editDescription.trim(),
      date: editEventDate,
    });
    await loadEvents();
    closeEditor();
  }

  async function addProject() {
    const title = projectInput.trim();
    if (!title) return;
    projectInput = "";
    projects = await invoke("add_project", { title });
  }

  function selectProject(id) {
    selectedProjectId = id;
    selectedTab = "todos";
    closeEditor();
  }

  function selectTab(tab) {
    selectedTab = tab;
    closeEditor();
  }

  function handleTodoKeydown(e) {
    if (e.key === "Enter") addTodo();
  }

  function handleProjectKeydown(e) {
    if (e.key === "Enter") addProject();
  }

  function handleEventKeydown(e) {
    if (e.key === "Enter") addEvent();
  }

  $effect(() => {
    loadProjects();
  });

  $effect(() => {
    if (selectedTab === "events") {
      loadEvents();
    } else {
      loadTodos();
    }
  });

  $effect(() => {
    if (editingTodoId !== null && !todos.some((todo) => todo.id === editingTodoId)) {
      closeEditor();
    }
    if (editingEventId !== null && !events.some((event) => event.id === editingEventId)) {
      closeEditor();
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
    {#if selectedProjectId !== null}
      <div class="tabs">
        <button
          class:active={selectedTab === "todos"}
          onclick={() => selectTab("todos")}
        >
          Todos
        </button>
        <button
          class:active={selectedTab === "events"}
          onclick={() => selectTab("events")}
        >
          Events
        </button>
      </div>
    {/if}
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
          <button class="secondary" onclick={closeEditor}>Back</button>
        </div>
      </section>
    {:else if editingEvent}
      <section class="edit-page">
        <h2>Edit event</h2>
        <label>
          Title
          <input type="text" bind:value={editTitle} />
        </label>
        <label>
          Date
          <input type="date" bind:value={editEventDate} />
        </label>
        <label>
          Description
          <textarea rows="4" bind:value={editDescription}></textarea>
        </label>
        <div class="edit-actions">
          <button onclick={saveEventEdits}>Save</button>
          <button class="secondary" onclick={closeEditor}>Back</button>
        </div>
      </section>
    {:else if selectedTab === "events"}
      <div class="event-input-row">
        <input
          type="text"
          placeholder="Event title"
          bind:value={eventTitleInput}
          onkeydown={handleEventKeydown}
        />
        <input type="date" bind:value={eventDateInput} />
        <button onclick={addEvent}>Add</button>
      </div>
      <ul class="event-list">
        {#each events as event (event.id)}
          <li>
            <div class="event-main">
              <span class="event-date">{event.date.slice(0, 10)}</span>
              <span>{event.title}</span>
            </div>
            <button class="edit" onclick={() => openEventEditor(event)}>Edit</button>
          </li>
        {/each}
      </ul>
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
