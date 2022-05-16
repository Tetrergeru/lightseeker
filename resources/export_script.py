import bpy
from mathutils import Matrix, Vector, Euler

ob = list(filter(lambda it: it.name == 'Cylinder', bpy.context.scene.objects))[0]
arm = ob.find_armature()
mesh = ob.to_mesh()
anim = arm.animation_data

def export_skl(arm, fname):
    with open(fname, "w") as file:
        for bone in arm.pose.bones:
            print(bone.name)

            file.write("b %s\n" % bone.name)
            if bone.parent:
                file.write("bp %s\n" % bone.parent.name)
            file.write("bb ")

            m = bone.bone.matrix_local
            m = arm.convert_space(pose_bone=bone,
                matrix=bone.matrix_basis,
                from_space='LOCAL',
                to_space='WORLD',
            )

            for a in matrix_decompose(m):
                file.write("%f " % a)
            file.write("\n\n")

def export_skin(mesh, arm, fname):
    i = 1
    bones = {}
    for bone in arm.pose.bones:
        bones[bone.name] = i
        i += 1
    with open(fname, "w") as file:
        for v in mesh.vertices:
            file.write("vl %d " % (v.index + 1))
            for g in v.groups:
                if not ob.vertex_groups[g.group].name in bones:
                    print("unknown group: ", g.group)
                file.write("%s %f " % (bones[ob.vertex_groups[g.group].name], g.weight))
            file.write("\n")

def export_anim(arm, fname):
    with open(fname, "w") as file:
        i = 0
        while i < 40:
            i += 2
            bpy.context.scene.frame_set(i)
            file.write("fr %d\n" % (i + 1))
            for bone in arm.pose.bones:
                print(bone.name)

                file.write("af ")
                m = bone.bone.matrix_local #bone.matrix

                m = arm.convert_space(pose_bone=bone,
                    matrix=bone.matrix_basis,
                    from_space='LOCAL',
                    to_space='WORLD',
                )

                for a in matrix_decompose(m):
                    file.write("%f " % a)
                file.write("\n")
            file.write("\n")

EPS = 0.0001

def matrix_decompose(mat):
    tr, rot, sc = mat.decompose()
    rot = rot.to_euler()#"XYZ"
    result = [
        -rot.x,
        -rot.z,
        rot.y,
    ]
    result.extend([
        tr.x,
        tr.z,
        -tr.y,
    ])
    return result

bpy.context.scene.frame_set(0)
export_skl(arm, "walk.skl")
export_skin(mesh, arm, "walk.skin")
export_anim(arm, "walk.anim")