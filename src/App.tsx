import { BrowserRouter, Routes, Route } from 'react-router-dom';
import AppLayout from './components/Layout';
import Dashboard from './pages/Dashboard';
import ToolsPage from './pages/ToolsPage';
import PackageStore from './pages/PackageStore';
import SystemClean from './pages/SystemClean';
import PortManager from './pages/PortManager';
import VersionManager from './pages/VersionManager';
import DiskAnalysis from './pages/DiskAnalysis';
import OrphanDeps from './pages/OrphanDeps';
import ProxySettings from './pages/ProxySettings';
import EnvManager from './pages/EnvManager';
import ProcessMonitor from './pages/ProcessMonitor';
import ProjectTemplates from './pages/ProjectTemplates';
import About from './pages/About';

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<AppLayout />}>
          <Route index element={<Dashboard />} />
          <Route path="tools" element={<ToolsPage />} />
          <Route path="store" element={<PackageStore />} />
          <Route path="clean" element={<SystemClean />} />
          <Route path="ports" element={<PortManager />} />
          <Route path="versions" element={<VersionManager />} />
          <Route path="disk" element={<DiskAnalysis />} />
          <Route path="orphan" element={<OrphanDeps />} />
          <Route path="proxy" element={<ProxySettings />} />
          <Route path="env" element={<EnvManager />} />
          <Route path="process" element={<ProcessMonitor />} />
          <Route path="templates" element={<ProjectTemplates />} />
          <Route path="about" element={<About />} />
        </Route>
      </Routes>
    </BrowserRouter>
  );
}

export default App;
