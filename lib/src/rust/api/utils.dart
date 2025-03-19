// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.9.0.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

Future<Float64List> readSalesData({required List<int> csvData}) =>
    RustLib.instance.api.crateApiUtilsReadSalesData(csvData: csvData);

Future<BigInt> getFrequency({required String frequency}) =>
    RustLib.instance.api.crateApiUtilsGetFrequency(frequency: frequency);
