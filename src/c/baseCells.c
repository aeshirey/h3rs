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

/**
 * @brief Given a base cell and the face it appears on, return
 *        the number of 60' ccw rotations for that base cell's
 *        coordinate system.
 * @returns The number of rotations, or INVALID_ROTATIONS if the base
 *          cell is not found on the given face
 */
int _baseCellToCCWrot60(int baseCell, int face) {
    if (face < 0 || face > NUM_ICOSA_FACES) return INVALID_ROTATIONS;
    for (int i = 0; i < 3; i++) {
        for (int j = 0; j < 3; j++) {
            for (int k = 0; k < 3; k++) {
                if (faceIjkBaseCells[face][i][j][k].baseCell == baseCell) {
                    return faceIjkBaseCells[face][i][j][k].ccwRot60;
                }
            }
        }
    }
    return INVALID_ROTATIONS;
}

/** @brief Return whether or not the tested face is a cw offset face.
 */
bool _baseCellIsCwOffset(int baseCell, int testFace) {
    return baseCellData[baseCell].cwOffsetPent[0] == testFace ||
           baseCellData[baseCell].cwOffsetPent[1] == testFace;
}


/**
 * res0IndexCount returns the number of resolution 0 indexes
 *
 * @return int count of resolution 0 indexes
 */
int H3_EXPORT(res0IndexCount)() { return NUM_BASE_CELLS; }

/**
 * getRes0Indexes generates all base cells storing them into the provided
 * memory pointer. Buffer must be of size NUM_BASE_CELLS * sizeof(H3Index).
 *
 * @param out H3Index* the memory to store the resulting base cells in
 */
void H3_EXPORT(getRes0Indexes)(H3Index* out) {
    for (int bc = 0; bc < NUM_BASE_CELLS; bc++) {
        H3Index baseCell = H3_INIT;
        H3_SET_MODE(baseCell, H3_HEXAGON_MODE);
        H3_SET_BASE_CELL(baseCell, bc);
        out[bc] = baseCell;
    }
}
