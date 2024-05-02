import { LitElement, css, html } from "lit";
import { customElement, property, state } from "lit/decorators.js";
import { AdminWebsocket, AppClient, AppWebsocket } from "@holochain/client";
import { provide } from "@lit-labs/context";
import "@material/mwc-circular-progress";

import { clientContext } from "./contexts.js";

import "./forum/posts/all-posts.js";
import "./forum/posts/create-post.js";

@customElement("holochain-app")
export class HolochainApp extends LitElement {
  @state() loading = true;

  @state() result: string | undefined;

  @provide({ context: clientContext })
  @property({ type: Object })
  client!: AppClient;

  error: any = undefined;

  async firstUpdated() {
    console.warn("HIII1");
    try {
      this.client = await AppWebsocket.connect();
      console.warn("HIII 2");
    } catch (e) {
      console.error("HI3", e);
      this.error = e;
    }

    this.loading = false;
  }

  render() {
    if (this.loading)
      return html`
        <mwc-circular-progress indeterminate></mwc-circular-progress>
      `;

    if (this.error) return html`<span>ERROR: ${this.error}</span>`;

    return html`
      <main>
        <h1>Forum</h1>

        <div id="content">
          <h2>All Posts</h2>
          <all-posts id="all-posts" style="margin-bottom: 16px"></all-posts>
          <create-post></create-post>
        </div>
      </main>
    `;
  }

  static styles = css`
    :host {
      min-height: 100vh;
      display: flex;
      flex-direction: column;
      align-items: center;
      justify-content: flex-start;
      font-size: calc(10px + 2vmin);
      color: #1a2b42;
      max-width: 960px;
      margin: 0 auto;
      text-align: center;
      background-color: var(--lit-element-background-color);
    }

    main {
      flex-grow: 1;
    }

    .app-footer {
      font-size: calc(12px + 0.5vmin);
      align-items: center;
    }

    .app-footer a {
      margin-left: 5px;
    }
  `;
}
