// press/lib/client/qspace_press/client_config.dart
//
// ─────────────────────────────────────────────────────────────────────────────
// CHANGELOG
// ─────────────────────────────────────────────────────────────────────────────
//   v1.2.0 — Fixed missing_required_argument on QAuthConfig.
//             Added tenantId and userClasses — both required by QAuthConfig
//             constructor. Defined two-tier reader/admin role structure
//             appropriate for a publishing platform.
//   v1.1.0 — Fixed all type errors from initial draft.
//             Removed unnecessary _StubSocialAdapter / _StubBiometricAdapter.
//             Replaced RestJwtAuthProvider with QAuthConfig().
//   v1.0.0 — Initial QSpace Press client config.
// ─────────────────────────────────────────────────────────────────────────────

import 'package:qspace_pages/qspace_pages.dart';

// No local stub classes needed — AppClientConfig defaults socialAdapter and
// biometricAdapter to StubSocialProvider / StubBiometricProvider internally
// when those params are omitted.

final kQSpacePressClientConfig = AppClientConfig(
  adapterType:     AuthAdapterType.restJwt,
  apiBaseUrl:      'https://api.qspacepress.com',
  defaultTenantId: 'qspace_press',
  auth: const QAuthConfig(
    tenantId: 'qspace_press',
    userClasses: [
      AuthUserClass(id: 'reader', label: 'Reader', role: QRole.user),
      AuthUserClass(id: 'admin',  label: 'Admin',  role: QRole.clientAdmin),
    ],
    loginHeading:    'Welcome to QSpace Press',
    loginSubheading: 'Sign in to continue',
    postLoginRoutes: {
      'reader': '/',
      'admin':  '/admin',
    },
  ),
  // overlay.json must be declared in press/pubspec.yaml under flutter: assets:
  localOverlayAssetPath: 'assets/overlay.json',
  // socialAdapter and biometricAdapter omitted —
  // core defaults to stubs for both when null.
);