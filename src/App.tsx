import { NBackground, NSTabs, NSTabsContent, NSTabsList, NSTabsTrigger } from '@nordstjerna/ui';

function App() {
  return (
    <>
      <NBackground />
      <NSTabs orientation="vertical">
        <NSTabsList>
          <NSTabsTrigger value="tab1">Tab 1</NSTabsTrigger>
          <NSTabsTrigger value="tab2">Tab 2</NSTabsTrigger>
          <NSTabsTrigger value="tab3">Tab 3</NSTabsTrigger>
        </NSTabsList>
        <NSTabsContent value="tab1">Tab 1 content</NSTabsContent>
        <NSTabsContent value="tab2">Tab 2 content</NSTabsContent>
        <NSTabsContent value="tab3">Tab 3 content</NSTabsContent>
      </NSTabs>
    </>
  );
}

export default App;
