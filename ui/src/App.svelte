<script>
  import { invoke } from "@tauri-apps/api/core";

  const PAGE_SIZE = 100;

  let projects = $state([]);
  let inboxCards = $state([]);
  let projectData = $state(null);
  let selectedProjectId = $state(null);
  let cardInput = $state("");
  let cardSectionInput = $state("");
  let projectInput = $state("");
  let sectionInput = $state("");
  let editingCardId = $state(null);
  let editingSectionId = $state(null);
  let editTitle = $state("");
  let editDescription = $state("");
  let editDeadline = $state("");
  let editCardSectionId = $state("");
  let editSectionTitle = $state("");

  const selectedProject = $derived(
    projectData?.project ?? projects.find((project) => project.id === selectedProjectId) ?? null,
  );
  const sectionGroups = $derived(projectData?.sections ?? []);
  const sections = $derived(sectionGroups.map((group) => group.section));
  const unsectionedCards = $derived(projectData?.cards_without_section ?? []);
  const cards = $derived(
    selectedProjectId === null
      ? inboxCards
      : [...unsectionedCards, ...sectionGroups.flatMap((group) => group.cards)],
  );
  const editingCard = $derived(cards.find((card) => card.id === editingCardId) ?? null);
  const editingSection = $derived(
    sections.find((section) => section.id === editingSectionId) ?? null,
  );

  const ignoreFilter = () => ({ kind: "ignore" });
  const noneFilter = () => ({ kind: "none" });

  async function loadProjects() {
    projects = await invoke("filter_projects", {
      limit: PAGE_SIZE,
      offset: 0,
    });
  }

  async function loadInboxCards() {
    inboxCards = await invoke("filter_cards", {
      limit: PAGE_SIZE,
      offset: 0,
      deadline: ignoreFilter(),
      projectId: noneFilter(),
      sectionId: ignoreFilter(),
    });
  }

  async function loadProjectData() {
    if (selectedProjectId === null) {
      projectData = null;
      return;
    }

    projectData = await invoke("get_project", {
      projectId: selectedProjectId,
    });
  }

  async function refreshSelectedView() {
    if (selectedProjectId === null) {
      await loadInboxCards();
    } else {
      await loadProjectData();
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
    await refreshSelectedView();
    cardSectionInput = "";
  }

  async function addProject() {
    const title = projectInput.trim();
    if (!title) return;

    projectInput = "";
    await invoke("add_project", { title });
    await loadProjects();
  }

  async function addSection() {
    const title = sectionInput.trim();
    if (!title || selectedProjectId === null) return;

    sectionInput = "";
    await invoke("add_section", {
      title,
      projectId: selectedProjectId,
    });
    await loadProjectData();
  }

  async function deleteCard(id) {
    await invoke("delete_card", { id });
    await refreshSelectedView();
  }

  async function deleteSection(id) {
    await invoke("delete_section", { id });
    await loadProjectData();
  }

  function openCardEditor(card) {
    editingCardId = card.id;
    editingSectionId = null;
    editTitle = card.title;
    editDescription = card.description ?? "";
    editDeadline = card.deadline ? card.deadline.slice(0, 10) : "";
    editCardSectionId = card.section_id ?? "";
    editSectionTitle = "";
  }

  function openSectionEditor(section) {
    editingSectionId = section.id;
    editingCardId = null;
    editSectionTitle = section.title;
    editTitle = "";
    editDescription = "";
    editDeadline = "";
    editCardSectionId = "";
  }

  function closeEditor() {
    editingCardId = null;
    editingSectionId = null;
    editTitle = "";
    editDescription = "";
    editDeadline = "";
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
    await refreshSelectedView();
    closeEditor();
  }

  async function saveSectionEdits() {
    const title = editSectionTitle.trim();
    if (!editingSectionId || !title) return;

    await invoke("update_section", {
      id: editingSectionId,
      title,
    });
    await loadProjectData();
    closeEditor();
  }

  function selectProject(id) {
    selectedProjectId = id;
    cardSectionInput = "";
    closeEditor();
  }

  function handleCardKeydown(event) {
    if (event.key === "Enter") addCard();
  }

  function handleProjectKeydown(event) {
    if (event.key === "Enter") addProject();
  }

  function handleSectionKeydown(event) {
    if (event.key === "Enter") addSection();
  }

  $effect(() => {
    loadProjects();
  });

  $effect(() => {
    refreshSelectedView();
  });

  $effect(() => {
    if (editingCardId !== null && !cards.some((card) => card.id === editingCardId)) {
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
          {#each inboxCards as card (card.id)}
            <li>
              <span>{card.title}</span>
              <button class="edit" onclick={() => openCardEditor(card)}>Edit</button>
              <button class="delete" onclick={() => deleteCard(card.id)}>&times;</button>
            </li>
          {/each}
        </ul>
      {:else if sectionGroups.length === 0}
        {#if unsectionedCards.length > 0}
          <ul class="card-list">
            {#each unsectionedCards as card (card.id)}
              <li>
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
                  <li>
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
                    <li>
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
