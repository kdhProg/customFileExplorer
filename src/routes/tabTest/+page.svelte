<script lang="ts">
    import { WebviewWindow } from '@tauri-apps/api/window';
  
    interface Tab {
      id: number;
      title: string;
      content: string;
    }
  
    let tabs: Tab[] = [
      { id: 1, title: "Tab 1", content: "This is content for Tab 1" },
      { id: 2, title: "Tab 2", content: "This is content for Tab 2" },
    ];
    
    let activeTab: Tab = tabs[0];
  
    function setActiveTab(tab: Tab) {
      activeTab = tab;
    }
  
    async function detachTab(tab: Tab) {
      // 새 Tauri 창을 생성
      const newWindow = new WebviewWindow(`detached-tab-${tab.id}`, {
        url: `detached_tab.html?tabId=${tab.id}`, // URL에 탭 ID 전달
        width: 800,
        height: 600,
      });
  
      // 생성된 창에 이벤트 리스너를 추가할 수 있음
      newWindow.once('tauri://created', () => {
        console.log(`Window detached-tab-${tab.id} created`);
      });
  
      newWindow.once('tauri://error', (e) => {
        console.error(`Failed to create window: ${e}`);
      });
    }
  </script>
  
  <div class="tabs">
    {#each tabs as tab (tab.id)}
      <button class="tab" on:click={() => setActiveTab(tab)} on:dblclick={() => detachTab(tab)}>
        {tab.title}
      </button>
    {/each}
  </div>
  
  <div class="content">
    <p>{activeTab.content}</p>
  </div>
  
  <style>
    .tabs {
      display: flex;
      background-color: #f2f2f2;
      padding: 5px;
    }
  
    .tab {
      padding: 10px;
      margin-right: 5px;
      cursor: pointer;
    }
  
    .tab:hover {
      background-color: #ddd;
    }
  
    .content {
      padding: 10px;
      border: 1px solid #ddd;
      margin-top: 10px;
    }
  </style>
  