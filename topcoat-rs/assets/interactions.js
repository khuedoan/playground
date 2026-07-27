const projectFilter = document.querySelector("[data-project-filter]");

if (projectFilter) {
  const projectCards = [...document.querySelectorAll("[data-project-card]")];
  const emptyState = document.querySelector("[data-project-empty]");

  const applyProjectFilter = () => {
    const query = projectFilter.value.trim().toLocaleLowerCase();
    let visibleCount = 0;

    for (const card of projectCards) {
      const matches = card.dataset.projectSearch.includes(query);
      card.hidden = !matches;
      visibleCount += Number(matches);
    }

    if (emptyState) {
      emptyState.hidden = visibleCount !== 0;
    }
  };

  projectFilter.addEventListener("input", applyProjectFilter);
  projectFilter.addEventListener("keydown", (event) => {
    if (event.key === "Escape" && projectFilter.value) {
      projectFilter.value = "";
      applyProjectFilter();
    }
  });
}

for (const switcher of document.querySelectorAll("[data-project-switcher]")) {
  const search = switcher.querySelector("[data-project-switcher-search]");
  if (!(search instanceof HTMLInputElement)) continue;

  const options = [
    ...switcher.querySelectorAll("[data-project-switcher-option]"),
  ];
  const emptyState = switcher.querySelector("[data-project-switcher-empty]");

  const applyProjectSwitcherFilter = () => {
    const query = search.value.trim().toLocaleLowerCase();
    let visibleCount = 0;

    for (const option of options) {
      const matches = (option.dataset.projectSearch ?? "").includes(query);
      option.hidden = !matches;
      visibleCount += Number(matches);
    }

    if (emptyState) {
      emptyState.hidden = visibleCount !== 0;
    }
  };

  search.addEventListener("input", applyProjectSwitcherFilter);
  search.addEventListener("keydown", (event) => {
    if (event.key === "Escape" && search.value) {
      event.stopPropagation();
      search.value = "";
      applyProjectSwitcherFilter();
    }
  });
}

const applyLogFilter = (viewer) => {
  const search = viewer.querySelector("[data-log-search]");
  const level = viewer.querySelector("[data-log-level]");
  const query =
    search instanceof HTMLInputElement
      ? search.value.trim().toLocaleLowerCase()
      : "";
  const selectedLevel =
    level instanceof HTMLSelectElement ? level.value.toUpperCase() : "ALL";
  let visibleCount = 0;

  for (const line of viewer.querySelectorAll("[data-log-line]")) {
    const matchesQuery =
      !query || (line.textContent ?? "").toLocaleLowerCase().includes(query);
    const matchesLevel =
      selectedLevel === "ALL" ||
      (line.dataset.logLevel ?? "").toUpperCase() === selectedLevel;
    const isVisible = matchesQuery && matchesLevel;

    line.hidden = !isVisible;
    visibleCount += Number(isVisible);
  }

  const count = viewer.querySelector("[data-log-count]");
  if (count) {
    count.textContent = `${visibleCount} ${visibleCount === 1 ? "line" : "lines"}`;
  }

  const emptyState = viewer.querySelector("[data-log-empty]");
  if (emptyState) {
    emptyState.hidden = visibleCount !== 0;
  }

  const output = viewer.querySelector("[data-log-output]");
  if (output) {
    output.hidden = visibleCount === 0;
  }
};

for (const viewer of document.querySelectorAll("[data-log-viewer]")) {
  const search = viewer.querySelector("[data-log-search]");
  const level = viewer.querySelector("[data-log-level]");

  search?.addEventListener("input", () => applyLogFilter(viewer));
  search?.addEventListener("keydown", (event) => {
    if (event.key === "Escape" && search.value) {
      search.value = "";
      applyLogFilter(viewer);
    }
  });
  level?.addEventListener("change", () => applyLogFilter(viewer));
  applyLogFilter(viewer);

  const output = viewer.querySelector("[data-log-output]");
  if (output instanceof HTMLElement) {
    output.scrollTop = output.scrollHeight;
  }
}

const positionDropdownPanel = (menu) => {
  if (!(menu instanceof HTMLDetailsElement) || !menu.open) return;

  const panel = menu.querySelector(":scope > div");
  if (!(panel instanceof HTMLElement)) return;

  const viewportPadding = 8;
  panel.style.maxWidth = `calc(100vw - ${viewportPadding * 2}px)`;
  panel.style.removeProperty("translate");

  const rect = panel.getBoundingClientRect();
  let offset = 0;

  if (rect.right > window.innerWidth - viewportPadding) {
    offset -= rect.right - (window.innerWidth - viewportPadding);
  }
  if (rect.left + offset < viewportPadding) {
    offset += viewportPadding - (rect.left + offset);
  }

  if (offset !== 0) {
    panel.style.translate = `${offset}px 0`;
  }
};

for (const menu of document.querySelectorAll("details[data-dropdown-menu]")) {
  menu.addEventListener("toggle", () => {
    if (menu.open) {
      requestAnimationFrame(() => positionDropdownPanel(menu));
    }
  });
}

window.addEventListener("resize", () => {
  for (const menu of document.querySelectorAll(
    "details[data-dropdown-menu][open]",
  )) {
    positionDropdownPanel(menu);
  }
});

const formControlSelector =
  "button, fieldset, input, optgroup, option, select, textarea";
const focusableFormControlSelector = [
  "button:not([disabled])",
  "input:not([type='hidden']):not([disabled])",
  "select:not([disabled])",
  "textarea:not([disabled])",
].join(", ");

const settingsFormSnapshots = new WeakMap();

const serializeSettingsForm = (form) =>
  [...form.elements]
    .filter(
      (control) =>
        control instanceof HTMLInputElement ||
        control instanceof HTMLSelectElement ||
        control instanceof HTMLTextAreaElement,
    )
    .map((control, index) => {
      const type =
        control instanceof HTMLInputElement ? control.type : control.tagName;
      const identity =
        control.name ||
        control.id ||
        control.dataset.variableKey ||
        control.dataset.variableValue ||
        `${control.tagName}-${index}`;
      const state =
        control instanceof HTMLInputElement &&
        (control.type === "checkbox" || control.type === "radio")
          ? String(control.checked)
          : control.value;

      return [type, identity, state].join("\u001f");
    })
    .join("\u001e");

const refreshSettingsFormState = (form) => {
  const initial = settingsFormSnapshots.get(form);
  if (initial === undefined) return;

  const isDirty = serializeSettingsForm(form) !== initial;
  form.dataset.settingsDirty = String(isDirty);

  const submit = form.querySelector("[data-settings-submit]");
  if (submit instanceof HTMLButtonElement) {
    submit.disabled = !isDirty;
  }

  const status = form.querySelector("[data-settings-status]");
  if (status) {
    status.textContent = isDirty ? "Unsaved changes" : "No unsaved changes";
  }
};

const markSettingsFormDirty = (element) => {
  const form =
    element instanceof HTMLFormElement
      ? element
      : element.closest("form[data-settings-form]");
  if (!(form instanceof HTMLFormElement)) return;

  refreshSettingsFormState(form);
};

const applyNetworkExposure = (select) => {
  const scope = select.form ?? document;
  const isPublic = select.value === "Public" && !select.disabled;

  for (const fields of scope.querySelectorAll(
    "[data-public-network-fields]",
  )) {
    fields.hidden = !isPublic;

    for (const control of fields.querySelectorAll(formControlSelector)) {
      control.disabled = !isPublic;
    }
  }
};

const applyStorageToggle = (toggle) => {
  const config = toggle.closest("[data-storage-config]");
  if (!config) return;

  const isEnabled = toggle.checked && !toggle.disabled;

  for (const fields of config.querySelectorAll("[data-storage-fields]")) {
    fields.hidden = !isEnabled;

    for (const control of fields.querySelectorAll(formControlSelector)) {
      control.disabled = !isEnabled;
    }
  }

  for (const addButton of config.querySelectorAll("[data-add-storage]")) {
    addButton.hidden = isEnabled;
  }
};

const managedDomain = (preview, componentName) => {
  const dnsLabel = (value, maxLength) =>
    value
      .toLocaleLowerCase()
      .replace(/[^a-z0-9]+/g, "-")
      .replace(/^-+|-+$/g, "")
      .slice(0, maxLength)
      .replace(/-+$/g, "") || "app";
  const component = dnsLabel(componentName, 24);
  const environment = dnsLabel(preview.dataset.domainEnvironment ?? "", 20);
  const tenant = dnsLabel(preview.dataset.domainTenant ?? "", 63);
  const project = dnsLabel(preview.dataset.domainProject ?? "", 63);
  const identity = [
    tenant,
    project,
    environment,
    component,
  ].join("/");
  let hash = 2166136261;

  for (const character of identity) {
    hash ^= character.charCodeAt(0);
    hash = Math.imul(hash, 16777619);
  }

  const suffix = (hash >>> 0)
    .toString(16)
    .padStart(6, "0")
    .slice(-6);
  return `${component}-${environment}-${suffix}.netamos.app`;
};

const updateManagedDomainPreview = (nameInput) => {
  const scope = nameInput.form ?? document;
  const preview = scope.querySelector("[data-managed-domain-preview]");
  if (!(preview instanceof HTMLElement)) return;

  preview.textContent = managedDomain(preview, nameInput.value);
};

const validateCustomDomain = (input) => {
  if (!(input instanceof HTMLInputElement)) return;

  const hostname = input.value.trim().replace(/[.]+$/g, "").toLocaleLowerCase();
  const registeredDomains = (input.dataset.registeredDomains ?? "")
    .split(",")
    .map((domain) => domain.trim().toLocaleLowerCase())
    .filter(Boolean);
  const isRegistered =
    !hostname ||
    registeredDomains.some(
      (domain) => hostname === domain || hostname.endsWith(`.${domain}`),
    );

  input.setCustomValidity(
    isRegistered ? "" : "Use a hostname under a verified tenant domain.",
  );
};

const validateEnvironmentName = (input) => {
  if (!(input instanceof HTMLInputElement)) return;

  const normalizedName = input.value
    .trim()
    .toLocaleLowerCase()
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-+|-+$/g, "");
  const existing = (input.dataset.existingEnvironments ?? "")
    .split(",")
    .map((slug) => slug.trim())
    .filter(Boolean);

  input.setCustomValidity(
    normalizedName && existing.includes(normalizedName)
      ? "An environment with this name already exists in the project."
      : "",
  );
};

const applyApplicationSourceKind = (select) => {
  const source = select.form?.querySelector("[data-application-source]");
  if (!(source instanceof HTMLInputElement)) return;

  source.placeholder =
    select.value === "image"
      ? "ghcr.io/owner/image:tag"
      : "https://github.com/owner/repository";
};

const updateNameFromSource = (source) => {
  const nameInput = source.form?.querySelector("[data-component-name]");
  if (!(nameInput instanceof HTMLInputElement)) return;

  const previousName = source.dataset.derivedComponentName ?? "";
  if (nameInput.value && nameInput.value !== previousName) return;

  const withoutSuffix = source.value
    .trim()
    .replace(/[?#].*$/, "")
    .replace(/\/+$/, "")
    .replace(/\.git$/i, "");
  const lastSegment = withoutSuffix.split("/").at(-1) ?? "";
  const derivedName = lastSegment
    .replace(/[@:].*$/, "")
    .toLocaleLowerCase()
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-+|-+$/g, "")
    .slice(0, 63)
    .replace(/-+$/g, "");

  if (!derivedName) {
    if (nameInput.value === previousName) {
      nameInput.value = "";
      delete source.dataset.derivedComponentName;
      updateManagedDomainPreview(nameInput);
    }
    return;
  }

  source.dataset.derivedComponentName = derivedName;
  nameInput.value = derivedName;
  updateManagedDomainPreview(nameInput);
};

const applyComponentKind = (select) => {
  const scope = select.form ?? document;
  const selectedKind = select.value;

  for (const section of scope.querySelectorAll(
    "[data-component-kind-fields]",
  )) {
    const isVisible = section.dataset.componentKindFields === selectedKind;
    section.hidden = !isVisible;
    if (section instanceof HTMLFieldSetElement) {
      section.disabled = !isVisible;
    }

    if (!isVisible) {
      for (const secret of section.querySelectorAll("[data-variable-value]")) {
        if (secret instanceof HTMLInputElement) {
          secret.value = "";
        }
      }
    }

    for (const control of section.querySelectorAll(formControlSelector)) {
      control.disabled = !isVisible;
    }
  }

  for (const exposure of scope.querySelectorAll("[data-network-exposure]")) {
    if (exposure instanceof HTMLSelectElement) {
      applyNetworkExposure(exposure);
    }
  }

  for (const toggle of scope.querySelectorAll("[data-storage-toggle]")) {
    if (toggle instanceof HTMLInputElement) {
      applyStorageToggle(toggle);
    }
  }
};

for (const select of document.querySelectorAll("[data-component-kind-select]")) {
  applyComponentKind(select);
}

for (const toggle of document.querySelectorAll("[data-storage-toggle]")) {
  if (toggle instanceof HTMLInputElement) {
    applyStorageToggle(toggle);
  }
}

for (const select of document.querySelectorAll("[data-network-exposure]")) {
  if (select instanceof HTMLSelectElement) {
    applyNetworkExposure(select);
  }
}

for (const select of document.querySelectorAll(
  "[data-application-source-kind]",
)) {
  if (select instanceof HTMLSelectElement) {
    applyApplicationSourceKind(select);
  }
}

for (const input of document.querySelectorAll("[data-component-name]")) {
  if (input instanceof HTMLInputElement) {
    updateManagedDomainPreview(input);
  }
}

for (const input of document.querySelectorAll("[data-custom-domain]")) {
  validateCustomDomain(input);
}

for (const input of document.querySelectorAll("[data-environment-name]")) {
  validateEnvironmentName(input);
}

for (const form of document.querySelectorAll("form[data-settings-form]")) {
  if (form instanceof HTMLFormElement) {
    settingsFormSnapshots.set(form, serializeSettingsForm(form));
    refreshSettingsFormState(form);
  }
}

const clearVariableKeyErrors = (editor) => {
  for (const keyInput of editor.querySelectorAll("[data-variable-key]")) {
    if (keyInput instanceof HTMLInputElement) {
      keyInput.setCustomValidity("");
    }
  }
};

document.addEventListener("click", (event) => {
  if (!(event.target instanceof Element)) return;

  for (const menu of document.querySelectorAll(
    "details[data-dropdown-menu][open]",
  )) {
    if (!menu.contains(event.target)) {
      menu.removeAttribute("open");
    }
  }

  const addStorageButton = event.target.closest("[data-add-storage]");
  if (addStorageButton) {
    const config = addStorageButton.closest("[data-storage-config]");
    const toggle = config?.querySelector("[data-storage-toggle]");

    if (toggle instanceof HTMLInputElement && !toggle.disabled) {
      event.preventDefault();
      toggle.checked = true;
      applyStorageToggle(toggle);
      markSettingsFormDirty(addStorageButton);

      const fields = config.querySelector("[data-storage-fields]:not([hidden])");
      const focusTarget = fields?.querySelector(focusableFormControlSelector);
      if (focusTarget instanceof HTMLElement) {
        focusTarget.focus();
      }
    }

    return;
  }

  const editVariableButton = event.target.closest("[data-edit-variable]");
  if (editVariableButton) {
    const row = editVariableButton.closest("[data-variable-row]");
    const fields = row?.querySelector("[data-variable-value-fields]");

    if (fields instanceof HTMLElement) {
      event.preventDefault();
      const isOpening = fields.hidden;
      fields.hidden = !isOpening;
      editVariableButton.setAttribute(
        "aria-expanded",
        isOpening ? "true" : "false",
      );
      editVariableButton.textContent = isOpening ? "Cancel" : "Replace";

      const valueInputs = [
        ...fields.querySelectorAll("[data-variable-value]"),
      ];
      for (const input of valueInputs) {
        if (
          input instanceof HTMLInputElement ||
          input instanceof HTMLTextAreaElement
        ) {
          input.disabled = !isOpening;
          if (!isOpening) {
            input.value = "";
          }
        }
      }

      if (isOpening) {
        const valueInput = valueInputs[0];
        if (valueInput instanceof HTMLElement) {
          valueInput.focus();
        }
      }
    }

    return;
  }

  const addVariableButton = event.target.closest("[data-add-variable]");
  if (addVariableButton) {
    const editor = addVariableButton.closest("[data-variable-editor]");
    const template = editor?.querySelector("template[data-variable-template]");

    if (template instanceof HTMLTemplateElement) {
      event.preventDefault();

      const rowFragment = template.content.cloneNode(true);
      const rows = [...rowFragment.querySelectorAll("[data-variable-row]")];
      const rowContainer = editor.querySelector("[data-variable-rows]");
      rowContainer?.append(rowFragment);

      rows[0]?.querySelector(focusableFormControlSelector)?.focus();
      markSettingsFormDirty(addVariableButton);

      const status = editor.querySelector("[data-variable-status]");
      if (status) {
        status.textContent = "Variable row added.";
      }
    }

    return;
  }

  const removeVariableButton = event.target.closest("[data-remove-variable]");
  if (removeVariableButton) {
    const row = removeVariableButton.closest("[data-variable-row]");
    const editor = row?.closest("[data-variable-editor]");
    if (row && editor) {
      event.preventDefault();
      const variableName =
        row.querySelector("[data-variable-key]")?.value || "Variable";
      const nextRow = row.nextElementSibling;
      const previousRow = row.previousElementSibling;
      row.remove();
      clearVariableKeyErrors(editor);
      markSettingsFormDirty(editor);

      const focusTarget =
        nextRow?.querySelector(focusableFormControlSelector) ??
        previousRow?.querySelector(focusableFormControlSelector) ??
        editor.querySelector("[data-add-variable]");
      if (focusTarget instanceof HTMLElement) {
        focusTarget.focus();
      }

      const status = editor.querySelector("[data-variable-status]");
      if (status) {
        status.textContent = `${variableName} removed.`;
      }
    }

    return;
  }

  const dismissButton = event.target.closest("[data-dismiss-feedback]");
  if (dismissButton) {
    dismissButton.closest("[data-feedback]")?.remove();
  }
});

document.addEventListener("change", (event) => {
  if (event.target instanceof Element) {
    markSettingsFormDirty(event.target);
  }

  if (
    event.target instanceof HTMLSelectElement &&
    event.target.matches("[data-component-kind-select]")
  ) {
    applyComponentKind(event.target);
  } else if (
    event.target instanceof HTMLSelectElement &&
    event.target.matches("[data-application-source-kind]")
  ) {
    applyApplicationSourceKind(event.target);
  } else if (
    event.target instanceof HTMLSelectElement &&
    event.target.matches("[data-network-exposure]")
  ) {
    applyNetworkExposure(event.target);
  } else if (
    event.target instanceof HTMLInputElement &&
    event.target.matches("[data-storage-toggle]")
  ) {
    applyStorageToggle(event.target);
  } else if (
    event.target instanceof HTMLInputElement &&
    event.target.matches("[data-custom-domain]")
  ) {
    event.target.value = event.target.value
      .trim()
      .replace(/[.]+$/g, "")
      .toLocaleLowerCase();
    validateCustomDomain(event.target);
  }
});

document.addEventListener("input", (event) => {
  if (event.target instanceof Element) {
    markSettingsFormDirty(event.target);
  }

  if (
    event.target instanceof HTMLInputElement &&
    event.target.matches("[data-component-name]")
  ) {
    updateManagedDomainPreview(event.target);
  } else if (
    event.target instanceof HTMLInputElement &&
    event.target.matches("[data-custom-domain]")
  ) {
    validateCustomDomain(event.target);
  } else if (
    event.target instanceof HTMLInputElement &&
    event.target.matches("[data-environment-name]")
  ) {
    validateEnvironmentName(event.target);
  } else if (
    event.target instanceof HTMLInputElement &&
    event.target.matches("[data-application-source]")
  ) {
    updateNameFromSource(event.target);
  } else if (
    event.target instanceof HTMLInputElement &&
    event.target.matches("[data-variable-key]")
  ) {
    const editor = event.target.closest("[data-variable-editor]");
    if (editor) {
      clearVariableKeyErrors(editor);
    }
  }
});

document.addEventListener("keydown", (event) => {
  if (event.key !== "Escape") return;

  const openMenus = [
    ...document.querySelectorAll("details[data-dropdown-menu][open]"),
  ];
  if (openMenus.length === 0) return;

  for (const menu of openMenus) {
    menu.removeAttribute("open");
  }

  const lastTrigger = openMenus.at(-1)?.querySelector("summary");
  if (lastTrigger instanceof HTMLElement) {
    lastTrigger.focus();
  }
});

document.addEventListener("submit", (event) => {
  if (!(event.target instanceof HTMLFormElement)) return;

  const serializedInput = event.target.querySelector(
    "input[data-variable-serialized]",
  );
  const editor = event.target.querySelector("[data-variable-editor]");

  if (serializedInput instanceof HTMLInputElement && editor) {
    // This prototype navigates with GET, so it carries keys only. Production
    // sends values in a POST body to the secret-writing API before redirecting.
    const sanitizeField = (value) => value.replace(/[\t\r\n]+/g, " ");
    const keys = [];
    const seenKeys = new Set();
    let duplicateKeyInput = null;
    const editorFieldset = editor.closest("fieldset");

    if (
      !(
        editorFieldset instanceof HTMLFieldSetElement &&
        editorFieldset.disabled
      )
    ) {
      for (const row of editor.querySelectorAll("[data-variable-row]")) {
        const keyInput = row.querySelector("input[data-variable-key]");
        const valueInput = row.querySelector("input[data-variable-value]");

        if (valueInput instanceof HTMLInputElement) {
          valueInput.removeAttribute("name");
        }
        if (!(keyInput instanceof HTMLInputElement) || keyInput.disabled) {
          continue;
        }

        const key = sanitizeField(keyInput.value).trim();
        keyInput.setCustomValidity("");
        if (key && seenKeys.has(key)) {
          keyInput.setCustomValidity("Variable keys must be unique.");
          duplicateKeyInput ??= keyInput;
        } else if (key) {
          seenKeys.add(key);
          keys.push(key);
        }
      }
    }

    if (duplicateKeyInput) {
      event.preventDefault();
      duplicateKeyInput.focus();
      duplicateKeyInput.reportValidity();
      return;
    }

    serializedInput.value = keys.join("\n");
  }

  const prompt = event.target.dataset.confirm;
  if (prompt && !window.confirm(prompt)) {
    event.preventDefault();
  }
});
