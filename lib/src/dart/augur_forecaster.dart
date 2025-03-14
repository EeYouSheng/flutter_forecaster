import 'dart:convert';

import 'package:flutter_forecaster/src/dart/model/prediction_source.dart';
import 'package:flutter_forecaster/src/dart/utils/base_forecaster.dart';
import 'package:flutter_forecaster/src/dart/model/prediction_type.dart';
import 'package:flutter_forecaster/src/rust/api/augurs.dart';

class AugurForecaster extends BaseForecaster {
  @override
  Future<List<PredictionSource>> initializePrediction({
    required List<PredictionSource> sourceData,
    PredictionType? prediction = PredictionType.monthly,
  }) async {
    List<PredictionSource> salePredictionList = [];

    final convertedData = convertToCsv(sourceData);

    final augurPredictionJson = await augursForecaster(
        csvData: convertedData, frequency: prediction!.name);

    final augurPredictionMap =
        jsonDecode(augurPredictionJson) as Map<String, dynamic>;

    try {
      final saleJsonList = augurPredictionMap['predictions'] as List<dynamic>;
      for (int i = 0; i < saleJsonList.length; i++) {
        final sale = saleJsonList[i];
        salePredictionList.add(
          PredictionSource(
              date: sourceData.last.date
                  .copyWith(month: sourceData.last.date.month + i + 1),
              value: sale as double),
        );
      }
    } catch (e) {
      rethrow;
    }

    return salePredictionList;
  }
}
