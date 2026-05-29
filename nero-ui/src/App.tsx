import { DialogOverlay } from "./components/ui/DialogManager";
import Layout from "./layouts/Layout";
import SettingsLayout from "./layouts/SettingsLayout";
import AppSettingsPage from "./pages/AppSettingsPage";
import ExtensionsSettingsPage from "./pages/ExtensionsSettingsPage";
import HomePage from "./pages/HomePage";
import SearchPage from "./pages/SearchPage";
import SeriesPage from "./pages/SeriesPage";
import StreamingSettingsPage from "./pages/StreamingSettingsPage";
import { Navigate, Route, Router } from "@solidjs/router";

export default function App() {
  return (
    <Router root={Layout}>
      <Route path="/" component={HomePage} />
      <Route path="/search" component={SearchPage} />
      <Route path="/series/:extensionId/:seriesId" component={SeriesPage} />

      <Route path="/settings" component={SettingsLayout}>
        <Route path="/" component={() => <Navigate href="/settings/app" />} />

        <Route path="/app" component={AppSettingsPage} />
        <Route path="/extensions" component={ExtensionsSettingsPage} />
        <Route path="/streaming" component={StreamingSettingsPage} />
      </Route>
      <DialogOverlay />
    </Router>
  );
}
