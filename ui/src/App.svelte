<script>
  import { invoke } from "@tauri-apps/api/core";

  let projects = $state([]);
  let cards = $state([]);
  let events = $state([]);
  let selectedProjectId = $state(null); // null = "No Project" view
  let selectedTab = $state("cards");
  let cardInput = $state("");
  let eventTitleInput = $state("");
  let eventDateInput = $state("");
  let projectInput = $state("");
  let editingCardId = $state(null);
  let editingEventId = $state(null);
  let editTitle = $state("");
  let editDescription = $state("");
  let editDeadline = $state("");
  let editEventDate = $state("");
  const editingCard = $derived(cards.find((card) => card.id === editingCardId) ?? null);
  const editingEvent = $derived(events.find((event) => event.id === editingEventId) ?? null);

  async function loadProjects() {
    projects = await invoke("get_projects");
  }

  async function loadCards() {
    if (selectedProjectId === null) {
      cards = await invoke("get_cards_without_project");
    } else {
      cards = await invoke("get_cards_by_project", { projectId: selectedProjectId });
    }
  }

  async function addCard() {
    const title = cardInput.trim();
    if (!title) return;
    cardInput = "";
    await invoke("add_card", {
      title,
      projectId: selectedProjectId,
    });
    await loadCards();
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

  async function toggleCard(id) {
    await invoke("toggle_card", { id });
    await loadCards();
  }

  async function deleteCard(id) {
    await invoke("delete_card", { id });
    await loadCards();
  }

  function openCardEditor(card) {
    editingCardId = card.id;
    editingEventId = null;
    editTitle = card.title;
    editDescription = card.description ?? "";
    editDeadline = card.deadline ? card.deadline.slice(0, 10) : "";
    editEventDate = "";
  }

  function openEventEditor(event) {
    editingEventId = event.id;
    editingCardId = null;
    editTitle = event.title;
    editDescription = event.description ?? "";
    editEventDate = event.date ? event.date.slice(0, 10) : "";
    editDeadline = "";
  }

  function closeEditor() {
    editingCardId = null;
    editingEventId = null;
    editTitle = "";
    editDescription = "";
    editDeadline = "";
    editEventDate = "";
  }

  async function saveCardEdits() {
    const title = editTitle.trim();
    if (!editingCardId || !title) return;
    await invoke("update_card", {
      id: editingCardId,
      title,
      description: editDescription.trim(),
      deadline: editDeadline || null,
    });
    await loadCards();
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
    selectedTab = "cards";
    closeEditor();
  }

  function selectTab(tab) {
    selectedTab = tab;
    closeEditor();
  }

  function handleCardKeydown(e) {
    if (e.key === "Enter") addCard();
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
      loadCards();
    }
  });

  $effect(() => {
    if (editingCardId !== null && !cards.some((card) => card.id === editingCardId)) {
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
          class:active={selectedTab === "cards"}
          onclick={() => selectTab("cards")}
        >
          Cards
        </button>
        <button
          class:active={selectedTab === "events"}
          onclick={() => selectTab("events")}
        >
          Events
        </button>
      </div>
    {/if}
    {#if editingCard}
      <section class="edit-page">
        <h2>Edit card</h2>
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
          <button onclick={saveCardEdits}>Save</button>
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
          bind:value={cardInput}
          onkeydown={handleCardKeydown}
        />
        <button onclick={addCard}>Add</button>
      </div>
      <ul class="card-list">
        {#each cards as card (card.id)}
          <li class:completed={card.completed}>
            <input
              type="checkbox"
              checked={card.completed}
              onchange={() => toggleCard(card.id)}
            />
            <span>{card.title}</span>
            <button class="edit" onclick={() => openCardEditor(card)}>Edit</button>
            <button class="delete" onclick={() => deleteCard(card.id)}>&times;</button>
          </li>
        {/each}
      </ul>
    {/if}
  </main>
</div>
