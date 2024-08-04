<script lang="ts">
    import type { Tag } from "@txtdot/dalet";
    import Element from "./tags/Element.svelte";
    import Heading from "./tags/Heading.svelte";
    import Paragraph from "./tags/Paragraph.svelte";
    import LineBreak from "./tags/LineBreak.svelte";
    import UnorderedList from "./tags/UnorderedList.svelte";
    import Link from "./tags/Link.svelte";
    import NavLink from "./tags/NavLink.svelte";
    import Button from "./tags/Button.svelte";
    import NavButton from "./tags/NavButton.svelte";
    import Blockquote from "./tags/Blockquote.svelte";
    import Code from "./tags/Code.svelte";
    import Pre from "./tags/Pre.svelte";

    export let tag: Tag;
</script>

{#if tag.id === 0}
    <Element {tag} />
{:else if tag.id === 1}
    <Heading {tag} />
{:else if tag.id === 2}
    <Paragraph {tag} />
{:else if tag.id === 3}
    <LineBreak />
{:else if tag.id === 4}
    <UnorderedList {tag} />
{:else if tag.id === 7}
    <Link {tag} />
{:else if tag.id === 8}
    <NavLink {tag} />
{:else if tag.id === 9}
    <Button {tag} />
{:else if tag.id === 10}
    <NavButton {tag} />
{:else if tag.id === 18}
    <Blockquote {tag} />
{:else if tag.id === 28}
    <Code {tag} />
{:else if tag.id === 29}
    <Pre {tag} />
{/if}
