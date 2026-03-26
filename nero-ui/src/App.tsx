import ExtensionGuardLayout from "./layouts/ExtensionGuardLayout";
import Layout from "./layouts/Layout";
import SettingsLayout from "./layouts/SettingsLayout";
import AppSettingsPage from "./pages/AppSettingsPage";
import ExtensionsSettingsPage from "./pages/ExtensionsSettingsPage";
import HomePage from "./pages/HomePage";
import SearchPage from "./pages/SearchPage";
import SeriesPage from "./pages/SeriesPage";
import { Navigate, Route, Router } from "@solidjs/router";

export default function App() {
  return (
    <Router root={Layout}>
      <Route path="/" component={HomePage} />

      <Route path="/" component={ExtensionGuardLayout}>
        <Route path="/search" component={SearchPage} />
        <Route path="/series/:seriesId" component={SeriesPage} />
      </Route>

      <Route path="/settings" component={SettingsLayout}>
        <Route path="/" component={() => <Navigate href="/settings/app" />} />

        <Route path="/app" component={AppSettingsPage} />
        <Route path="/extensions" component={ExtensionsSettingsPage} />
      </Route>
    </Router>
  );
}
