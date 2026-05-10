import Layout from "./layouts/Layout";
import SettingsLayout from "./layouts/SettingsLayout";
import AppSettingsPage from "./pages/AppSettingsPage";
import ExtensionsSettingsPage from "./pages/ExtensionsSettingsPage";
import HomePage from "./pages/HomePage";
import ProcessorSettingsPage from "./pages/ProcessorSettingsPage";
import SearchPage from "./pages/SearchPage";
import SeriesPage from "./pages/SeriesPage";
import { AppPreferencesProvider } from "./providers/AppPreferencesProvider";
import { ExtensionPreferencesProvider } from "./providers/ExtensionPreferencesProvider";
import { Navigate, Route, Router } from "@solidjs/router";

export default function App() {
  return (
    <AppPreferencesProvider>
      <ExtensionPreferencesProvider>
        <Router root={Layout}>
          <Route path="/" component={HomePage} />
          <Route path="/search" component={SearchPage} />
          <Route path="/series/:seriesId" component={SeriesPage} />

          <Route path="/settings" component={SettingsLayout}>
            <Route
              path="/"
              component={() => <Navigate href="/settings/app" />}
            />

            <Route path="/app" component={AppSettingsPage} />
            <Route path="/extensions" component={ExtensionsSettingsPage} />
            <Route path="/processor" component={ProcessorSettingsPage} />
          </Route>
        </Router>
      </ExtensionPreferencesProvider>
    </AppPreferencesProvider>
  );
}
