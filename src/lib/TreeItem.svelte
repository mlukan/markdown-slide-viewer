<script lang="ts">
  import { createEventDispatcher } from "svelte";

  export type TreeNode = {
    name: string;
    path: string;
    is_dir: boolean;
    children: TreeNode[];
  };

  export let node: TreeNode;
  export let expanded: Set<string>;
  export let isOpen: (path: string) => boolean;
  export let isActive: (path: string) => boolean;

  const dispatch = createEventDispatcher<{ openFile: string; toggle: string }>();

  function onToggle(path: string) {
    dispatch("toggle", path);
  }

  function onOpenFile(path: string) {
    dispatch("openFile", path);
  }
</script>

<li class="tree-node">
  {#if node.is_dir}
    <button class="tree-label" on:click={() => onToggle(node.path)}>
      {isOpen(node.path) ? "▾" : "▸"} {node.name}
    </button>
    {#if isOpen(node.path)}
      <ul>
        {#each node.children as child}
          <svelte:self
            node={child}
            {expanded}
            {isOpen}
            {isActive}
            on:openFile
            on:toggle
          />
        {/each}
      </ul>
    {/if}
  {:else}
    <button class="tree-label" class:active={isActive(node.path)} on:click={() => onOpenFile(node.path)}>
      {node.name}
    </button>
  {/if}
</li>
