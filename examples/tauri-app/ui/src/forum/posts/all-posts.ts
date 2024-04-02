import { LitElement, html } from 'lit';
import { state, customElement, property } from 'lit/decorators.js';
import { AppAgentClient, AgentPubKey, Link, EntryHash, ActionHash, Record, NewEntryAction } from '@holochain/client';
import { consume } from '@lit-labs/context';
import { Task } from '@lit-labs/task';
import '@material/mwc-circular-progress';
import './post-detail.js';

import { clientContext } from '../../contexts';
import { PostsSignal } from './types';


@customElement('all-posts')
export class AllPosts extends LitElement {
  @consume({ context: clientContext, subscribe: true })
  client!: AppAgentClient;


  @state()
  signaledHashes: Array<ActionHash> = [];

  _fetchPosts = new Task(this, ([]) => this.client.callZome({
    cap_secret: null,
    role_name: 'forum',
    zome_name: 'posts',
    fn_name: 'get_all_posts',
    payload: null,
  }) as Promise<Array<Link>>, () => []);

  firstUpdated() {

    this.client.on('signal', signal => {
      if (signal.zome_name !== 'posts') return;
      const payload = signal.payload as PostsSignal;
      if (payload.type !== 'EntryCreated') return;
      this.signaledHashes = [payload.action.hashed.hash, ...this.signaledHashes];
    });
  }

  renderList(hashes: Array<ActionHash>) {
    if (hashes.length === 0) return html`<span>No posts found.</span>`;

    return html`
      <div style="display: flex; flex-direction: column; gap: 16px">
      ${hashes.map(hash => html`
        <post-detail .postHash=${hash}></post-detail>
      `)}
      </div>
    `;
  }

  render() {
    return this._fetchPosts.render({
      pending: () => html`
        <div style="display: flex; flex: 1; align-items: center; justify-content: center">
          <mwc-circular-progress indeterminate></mwc-circular-progress>
        </div>
      `,
      complete: (links) => this.renderList([...this.signaledHashes, ...links.map(l => l.target)]),
      error: (e: any) => html`<span>Error fetching the posts: ${e}.</span>`
    });
  }
}
