/*
 * Copyright 2016-2020 Uber Technologies, Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *         http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
/** @file baseCells.c
 * @brief   Base cell related lookup tables and access functions.
 */

#include "baseCells.h"

#include "h3Index.h"




/** @brief Find base cell given FaceIJK.
 *
 * Given the face number and a resolution 0 ijk+ coordinate in that face's
 * face-centered ijk coordinate system, return the base cell located at that
 * coordinate.
 *
 * Valid ijk+ lookup coordinates are from (0, 0, 0) to (2, 2, 2).
 */
int _faceIjkToBaseCell(const FaceIJK* h) {
    return faceIjkBaseCells[h->face][h->coord.i][h->coord.j][h->coord.k]
        .baseCell;
}

/** @brief Find base cell given FaceIJK.
 *
 * Given the face number and a resolution 0 ijk+ coordinate in that face's
 * face-centered ijk coordinate system, return the number of 60' ccw rotations
 * to rotate into the coordinate system of the base cell at that coordinates.
 *
 * Valid ijk+ lookup coordinates are from (0, 0, 0) to (2, 2, 2).
 */
int _faceIjkToBaseCellCCWrot60(const FaceIJK* h) {
    return faceIjkBaseCells[h->face][h->coord.i][h->coord.j][h->coord.k]
        .ccwRot60;
}

/** @brief Find the FaceIJK given a base cell.
 */
void _baseCellToFaceIjk(int baseCell, FaceIJK* h) {
    *h = baseCellData[baseCell].homeFijk;
}


/** @brief Return whether or not the tested face is a cw offset face.
 */
bool _baseCellIsCwOffset(int baseCell, int testFace) {
    return baseCellData[baseCell].cwOffsetPent[0] == testFace ||
           baseCellData[baseCell].cwOffsetPent[1] == testFace;
}

