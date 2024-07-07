<script lang="ts">
  import {onMount, getContext } from 'svelte';
  import '@material/mwc-circular-progress';
  import type { Record, ActionHash, AppClient, EntryHash, AgentPubKey, DnaHash, CellInfo, NetworkInfoResponse } from '@holochain/client';
  import { clientContext } from '../../contexts';
  import { flatten } from 'lodash';
    
  let client: AppClient = (getContext(clientContext) as any).getClient();
  
  let networkInfo: NetworkInfoResponse;
  let lastUpdated: Date;


  let loading: boolean;
  let error: any = undefined;
  
  onMount(async () => {
    setInterval(fetchNetworkInfo, 10000);
    await fetchNetworkInfo();
  });
  
  async function fetchNetworkInfo() {
    loading = true;
    
    try {
      const appInfo = await client.appInfo();
      console.log(appInfo);
      console.log(Object.values(appInfo.cell_info));
      const dnaHashes = flatten(Object.values(appInfo.cell_info).map((cellinfos) => cellinfos.map((cell: CellInfo) => (cell as any).provisioned.cell_id[0])));
      console.log(dnaHashes);

      networkInfo = await client.networkInfo({
        dnas: dnaHashes
      });
      lastUpdated = new Date();
      console.log(networkInfo);

      
    } catch (e) {
      error = e;
    }
  
    loading = false;
  }
</script>
  
{#if networkInfo}
<div>
  <div style="display: flex; flex-direction: column">
    {JSON.stringify(networkInfo)}
  </div>
  <div>
    Updated {lastUpdated}
  </div>
</div>
{/if}