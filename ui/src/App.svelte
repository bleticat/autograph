<script>
  import { invoke } from "@tauri-apps/api/core";

  let projects = $state([]);
  let sections = $state([]);
  let cards = $state([]);
  let events = $state([]);
  let selectedProjectId = $state(null);
  let selectedTab = $state("cards");
  let cardInput = $state("");
  let cardSectionInput = $state("");
  let eventTitleInput = $state("");
  let eventDateInput = $state("");
  let projectInput = $state("");
  let sectionInput = $state("");
  let editingCardId = $state(null);
  let editingEventId = $state(null);
  let editingSectionId = $state(null);
  let editTitle = $state("");
  let editDescription = $state("");
  let editDeadline = $state("");
  let editEventDate = $state("");
  let editCardSectionId = $state("");
  let editSectionTitle = $state("");

  const selectedProject = $derived(
    projects.find((project) => project.id === selectedProjectId) ?? null,
  );
  const editingCard = $derived(cards.find((card) => card.id === editingCardId) ?? null);
  const editingEvent = $derived(events.find((event) => event.id === editingEventId) ?? null);
  const editingSection = $derived(
    sections.find((section) => section.id === editingSectionId) ?? null,
  );
  const unsectionedCards = $derived(cards.filter((card) => card.section_id === null));
  const sectionGroups = $derived(
    sections.map((section) => ({
      section,
      cards: cards.filter((card) => card.section_id === section.id),
    })),
  );

  async function loadProjects() {
    projects = await invoke("get_projects");
  }

  async function loadSections() {
    if (selectedProjectId === null) {
      sections = [];
      return;
    }

    sections = await invoke("get_sections_by_project", {
      projectId: selectedProjectId,
    });
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
      sectionId: selectedProjectId === null ? null : cardSectionInput || null,
    });
    await loadCards();
    cardSectionInput = "";
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

  async function addProject() {
    const title = projectInput.trim();
    if (!title) return;

    projectInput = "";
    projects = await invoke("add_project", { title });
  }

  async function addSection() {
    const title = sectionInput.trim();
    if (!title || selectedProjectId === null) return;

    sectionInput = "";
    await invoke("add_section", {
      title,
      projectId: selectedProjectId,
    });
    await loadSections();
  }

  async function toggleCard(id) {
    await invoke("toggle_card", { id });
    await loadCards();
  }

  async function deleteCard(id) {
    await invoke("delete_card", { id });
    await loadCards();
  }

  async function deleteSection(id) {
    await invoke("delete_section", { id });
    await Promise.all([loadSections(), loadCards()]);
  }

  function openCardEditor(card) {
    editingCardId = card.id;
    editingEventId = null;
    editingSectionId = null;
    editTitle = card.title;
    editDescription = card.description ?? "";
    editDeadline = card.deadline ? card.deadline.slice(0, 10) : "";
    editEventDate = "";
    editCardSectionId = card.section_id ?? "";
    editSectionTitle = "";
  }

  function openEventEditor(event) {
    editingEventId = event.id;
    editingCardId = null;
    editingSectionId = null;
    editTitle = event.title;
    editDescription = event.description ?? "";
    editEventDate = event.date ? event.date.slice(0, 10) : "";
    editDeadline = "";
    editCardSectionId = "";
    editSectionTitle = "";
  }

  function openSectionEditor(section) {
    editingSectionId = section.id;
    editingCardId = null;
    editingEventId = null;
    editSectionTitle = section.title;
    editTitle = "";
    editDescription = "";
    editDeadline = "";
    editEventDate = "";
    editCardSectionId = "";
  }

  function closeEditor() {
    editingCardId = null;
    editingEventId = null;
    editingSectionId = null;
    editTitle = "";
    editDescription = "";
    editDeadline = "";
    editEventDate = "";
    editCardSectionId = "";
    editSectionTitle = "";
  }

  async function saveCardEdits() {
    const title = editTitle.trim();
    if (!editingCardId || !title) return;

    await invoke("update_card", {
      id: editingCardId,
      title,
      description: editDescription.trim(),
      deadline: editDeadline || null,
      projectId: selectedProjectId,
      sectionId: selectedProjectId === null ? null : editCardSectionId || null,
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

  async function saveSectionEdits() {
    const title = editSectionTitle.trim();
    if (!editingSectionId || !title) return;

    await invoke("update_section", {
      id: editingSectionId,
      title,
    });
    await loadSections();
    closeEditor();
  }

  function selectProject(id) {
    selectedProjectId = id;
    selectedTab = "cards";
    cardSectionInput = "";
    closeEditor();
  }

  function selectTab(tab) {
    selectedTab = tab;
    closeEditor();
  }

  function handleCardKeydown(event) {
    if (event.key === "Enter") addCard();
  }

  function handleProjectKeydown(event) {
    if (event.key === "Enter") addProject();
  }

  function handleEventKeydown(event) {
    if (event.key === "Enter") addEvent();
  }

  function handleSectionKeydown(event) {
    if (event.key === "Enter") addSection();
  }

  $effect(() => {
    loadProjects();
  });

  $effect(() => {
    loadSections();
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
    if (editingSectionId !== null && !sections.some((section) => section.id === editingSectionId)) {
      closeEditor();
    }
  });

  $effect(() => {
    if (selectedProjectId === null) {
      cardSectionInput = "";
      editCardSectionId = "";
      return;
    }

    if (cardSectionInput && !sections.some((section) => section.id === cardSectionInput)) {
      cardSectionInput = "";
    }

    if (editCardSectionId && !sections.some((section) => section.id === editCardSectionId)) {
      editCardSectionId = "";
    }
  });
</script>

<div class="layout">
  <aside class="sidebar">
    <h2>Projects</h2>
    <ul class="project-list">
      <li class:active={selectedProjectId === null}>
        <button type="button" onclick={() => selectProject(null)}>Inbox</button>
      </li>
      {#each projects as project (project.id)}
        <li class:active={selectedProjectId === project.id}>
          <button type="button" onclick={() => selectProject(project.id)}>
            {project.title}
          </button>
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
    <div class="main-header">
      <div>
        <p class="eyebrow">{selectedProjectId === null ? "Personal space" : "Project"}</p>
        <h1>{selectedProject?.title ?? "Inbox"}</h1>
      </div>
      {#if selectedProjectId !== null}
        <div class="tabs">
          <button
            class:active={selectedTab === "cards"}
            onclick={() => selectTab("cards")}
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
    </div>

    {#if editingCard}
      <section class="edit-page">
        <h2>Edit todo</h2>
        <label>
          Title
          <input type="text" bind:value={editTitle} />
        </label>
        <label>
          Description
          <textarea rows="4" bind:value={editDescription}></textarea>
        </label>
        {#if selectedProjectId !== null}
          <label>
            Section
            <select bind:value={editCardSectionId}>
              <option value="">No section</option>
              {#each sections as section (section.id)}
                <option value={section.id}>{section.title}</option>
              {/each}
            </select>
          </label>
        {/if}
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
    {:else if editingSection}
      <section class="edit-page">
        <h2>Edit section</h2>
        <label>
          Title
          <input type="text" bind:value={editSectionTitle} />
        </label>
        <div class="edit-actions">
          <button onclick={saveSectionEdits}>Save</button>
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
      <div class="todo-toolbar">
        <div class="input-row">
          <input
            type="text"
            placeholder={selectedProjectId === null
              ? "What needs to be done?"
              : "Add a task to this project"}
            bind:value={cardInput}
            onkeydown={handleCardKeydown}
          />
          {#if selectedProjectId !== null}
            <select bind:value={cardSectionInput}>
              <option value="">No section</option>
              {#each sections as section (section.id)}
                <option value={section.id}>{section.title}</option>
              {/each}
            </select>
          {/if}
          <button onclick={addCard}>Add</button>
        </div>

        {#if selectedProjectId !== null}
          <div class="section-input-row">
            <input
              type="text"
              placeholder="New section…"
              bind:value={sectionInput}
              onkeydown={handleSectionKeydown}
            />
            <button onclick={addSection}>Add section</button>
          </div>
        {/if}
      </div>

      {#if selectedProjectId === null}
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
      {:else if sections.length === 0}
        {#if cards.length > 0}
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
        {:else}
          <p class="empty-state">This project is empty. Add a task or create a section to get started.</p>
        {/if}
      {:else}
        <div class="section-board">
          {#if unsectionedCards.length > 0}
            <section class="todo-group">
              <div class="group-header">
                <div>
                  <h2>No section</h2>
                  <p>Tasks added directly to the project</p>
                </div>
                <span class="count-pill">{unsectionedCards.length}</span>
              </div>
              <ul class="card-list">
                {#each unsectionedCards as card (card.id)}
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
            </section>
          {/if}

          {#each sectionGroups as group (group.section.id)}
            <section class="todo-group">
              <div class="group-header">
                <div>
                  <h2>{group.section.title}</h2>
                  <p>Organize focused work inside the project</p>
                </div>
                <div class="group-controls">
                  <span class="count-pill">{group.cards.length}</span>
                  <button class="edit" onclick={() => openSectionEditor(group.section)}>Edit</button>
                  <button class="delete" onclick={() => deleteSection(group.section.id)}>&times;</button>
                </div>
              </div>

              {#if group.cards.length > 0}
                <ul class="card-list">
                  {#each group.cards as card (card.id)}
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
              {:else}
                <p class="empty-group">No tasks here yet.</p>
              {/if}
            </section>
          {/each}
        </div>
      {/if}
    {/if}
  </main>
</div>
